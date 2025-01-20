use crate::rpc::{RpcClient, RpcServer};
use crate::storage::Storage;
use crate::types::Distance;
use crate::{Key, NodeId, RoutingTable, ALPHA, K};
use anyhow::Result;
use std::collections::HashSet;
use std::net::SocketAddr;

pub struct Node {
    id: NodeId,
    addr: SocketAddr,
    routing_table: RoutingTable,
    storage: Storage,
    rpc_server: RpcServer,
    rpc_client: RpcClient,
}

impl Node {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        let id = NodeId::random();
        let routing_table = RoutingTable::new(id);
        let storage = Storage::new();
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

    pub async fn store(&mut self, key: Key, value: Vec<u8>) -> Result<()> {
        // Find nodes to store the value
        let nodes = self.lookup_nodes(key).await?;
        for (node_id, addr) in nodes {
            self.rpc_client
                .store(self.id, node_id, addr, key, value.clone())
                .await?;
        }
        Ok(())
    }

    pub async fn lookup_nodes(&self, key: Key) -> Result<Vec<(NodeId, SocketAddr)>> {
        let mut closest = self.routing_table.closest_nodes(&key, K);
        let mut contacted = HashSet::new();
        let mut pending = HashSet::new();
        let mut closest_with_addr = Vec::new(); // (NodeId, SocketAddr) pairs

        while !closest.is_empty() {
            let mut concurrent_lookups = Vec::new();

            // Query up to ALPHA nodes in parallel
            for node_id in closest.iter().take(ALPHA) {
                if !contacted.contains(node_id) && !pending.contains(node_id) {
                    // TODO: Get actual SocketAddr from routing table
                    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
                    pending.insert(*node_id);
                    let lookup = self.rpc_client.find_node(self.id, key, addr);
                    concurrent_lookups.push(lookup);
                }
            }

            // Wait for responses
            let responses = futures::future::join_all(concurrent_lookups).await;

            // Process responses
            for response in responses {
                if let Ok(new_nodes) = response {
                    // new_nodesから(NodeId, SocketAddr)のペアを処理
                    for (node_id, addr) in new_nodes {
                        closest.push(node_id);
                        closest_with_addr.push((node_id, addr));
                    }
                }
            }

            // Sort results by distance and limit to K nodes
            closest.sort_by_key(|n| Distance::between(n, &key));
            closest.truncate(K);

            closest_with_addr.sort_by_key(|(n, _)| Distance::between(n, &key));
            closest_with_addr.truncate(K);

            // Update contacted and pending sets
            for node_id in closest.iter().take(ALPHA) {
                pending.remove(node_id);
                contacted.insert(*node_id);
            }
        }

        Ok(closest_with_addr)
    }

    pub async fn run(&self) -> Result<()> {
        self.rpc_server.start(self.id).await
    }
}
