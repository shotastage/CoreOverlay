use crate::rpc::{RpcClient, RpcServer};
use crate::storage::Storage;
use crate::types::Distance;
use crate::{Key, NodeId, RoutingTable, ALPHA, K};
use anyhow::Result;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

/// A node in the Kademlia distributed hash table network.
///
/// Each node maintains:
/// - A unique 160-bit identifier (NodeId)
/// - A routing table containing information about other nodes
/// - Persistent storage for key-value pairs using Sled
/// - RPC capabilities for network communication
///
/// # References
/// Based on the Kademlia DHT paper:
/// "Kademlia: A Peer-to-peer Information System Based on the XOR Metric"
/// by Petar Maymounkov and David Mazières
pub struct Node {
    /// Unique 160-bit identifier for this node
    id: NodeId,
    /// Network address of this node
    addr: SocketAddr,
    /// k-bucket routing table storing known nodes
    routing_table: RoutingTable,
    /// Persistent key-value storage using Sled
    storage: Storage,
    /// Server for handling incoming RPCs
    rpc_server: RpcServer,
    /// Client for making outgoing RPCs
    rpc_client: RpcClient,
}

impl Node {
    /// Creates a new Kademlia node with a randomly generated NodeId.
    ///
    /// # Arguments
    /// * `addr` - The socket address this node will listen on
    /// * `storage_path` - Path to the directory where Sled will store its data
    ///
    /// # Returns
    /// * `Result<Self>` - A new Node instance or an error
    pub async fn new(addr: SocketAddr, storage_path: impl AsRef<Path>) -> Result<Self> {
        let id = NodeId::random();
        let routing_table = RoutingTable::new(id);
        let storage = Storage::new(storage_path)?;
        let rpc_server = RpcServer::new().await?;
        let rpc_client = RpcClient::new().await?;

        Ok(Node {
            id,
            addr,
            routing_table,
            storage,
            rpc_server,
            rpc_client,
        })
    }

    /// Stores a value in the DHT network.
    ///
    /// This implements the Kademlia STORE operation. The value is stored on the k nodes
    /// closest to the key in the XOR metric space.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The value to store
    ///
    /// # Process
    /// 1. Looks up the k closest nodes to the key
    /// 2. Sends STORE RPCs to each of these nodes
    pub async fn store(&mut self, key: Key, value: Vec<u8>) -> Result<()> {
        // First store locally with a TTL of 24 hours
        self.storage.store(
            key,
            value.clone(),
            std::time::Duration::from_secs(24 * 60 * 60),
        )?;

        // Then replicate to k closest nodes
        let nodes = self.lookup_nodes(key).await?;
        for (node_id, addr) in nodes {
            self.rpc_client
                .store(self.id, node_id, addr, key, value.clone())
                .await?;
        }
        Ok(())
    }

    /// Looks up the k closest nodes to a given key using the Kademlia node lookup algorithm.
    ///
    /// This is a core operation in Kademlia that implements the iterative node lookup process:
    /// 1. Starts with the α closest nodes from the local routing table
    /// 2. Sends parallel FIND_NODE RPCs to improve knowledge of the target ID's neighborhood
    /// 3. Iteratively queries nodes that are closer to the target until no closer nodes are found
    ///
    /// # Arguments
    /// * `key` - The target key to find nodes close to
    ///
    /// # Returns
    /// * `Result<Vec<(NodeId, SocketAddr)>>` - Vector of the k closest nodes and their addresses
    ///
    /// # Implementation Details
    /// * Uses α (ALPHA) parallel lookups for better performance
    /// * Maintains sets of contacted and pending nodes to avoid duplicate queries
    /// * Sorts results by XOR distance to the target key
    /// * Terminates when no closer nodes are found
    pub async fn lookup_nodes(&self, key: Key) -> Result<Vec<(NodeId, SocketAddr)>> {
        let mut closest = self.routing_table.closest_nodes(&key, K);
        let mut contacted = HashSet::new();
        let mut pending = HashSet::new();
        let mut closest_with_addr = Vec::new();

        while !closest.is_empty() {
            let mut concurrent_lookups = Vec::new();

            // Query up to ALPHA nodes in parallel for better network utilization
            for node_id in closest.iter().take(ALPHA) {
                if !contacted.contains(node_id) && !pending.contains(node_id) {
                    // TODO: Get actual SocketAddr from routing table
                    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
                    pending.insert(*node_id);
                    let lookup = self.rpc_client.find_node(self.id, key, addr);
                    concurrent_lookups.push(lookup);
                }
            }

            // Process responses and update closest nodes
            let responses = futures::future::join_all(concurrent_lookups).await;
            for response in responses {
                if let Ok(new_nodes) = response {
                    for (node_id, addr) in new_nodes {
                        closest.push(node_id);
                        closest_with_addr.push((node_id, addr));
                    }
                }
            }

            // Maintain k-closest nodes invariant
            closest.sort_by_key(|n| Distance::between(n, &key));
            closest.truncate(K);

            closest_with_addr.sort_by_key(|(n, _)| Distance::between(n, &key));
            closest_with_addr.truncate(K);

            // Track queried nodes
            for node_id in closest.iter().take(ALPHA) {
                pending.remove(node_id);
                contacted.insert(*node_id);
            }
        }

        Ok(closest_with_addr)
    }

    /// Starts the node's RPC server to handle incoming requests.
    ///
    /// This method runs indefinitely, processing incoming RPCs according to the
    /// Kademlia protocol specification.
    pub async fn run(&self) -> Result<()> {
        self.rpc_server.start(self.id).await
    }
}
