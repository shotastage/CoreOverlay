//! Implementation of Kademlia's routing table using k-buckets.
//!
//! The routing table is a key component of the Kademlia DHT, organizing known nodes
//! based on their XOR distance from the local node. It uses a binary tree-like structure
//! where each k-bucket stores up to k nodes with specific distance properties.

use crate::KEY_SIZE;
use crate::{Distance, NodeId, K};
use std::collections::VecDeque;
use std::net::SocketAddr;
use tokio::time::Instant;

/// Information about a node in the Kademlia network.
///
/// This struct contains all necessary information to identify and communicate with
/// a node in the Kademlia DHT network:
/// - A unique node identifier (NodeId)
/// - Network address information (SocketAddr)
/// - Timestamp of last successful contact
///
/// # Fields
/// * `node_id` - The unique 160-bit identifier of the node
/// * `sock_addr` - The network address (IP and port) used for communication
/// * `last_seen` - Timestamp of the last successful contact with this node
///
#[derive(Clone)]
pub struct NodeInfo {
    /// The unique 160-bit identifier of the node
    pub node_id: NodeId,
    /// The network address (IP and port) used for communication
    pub sock_addr: SocketAddr,
    /// Timestamp of the last successful contact with this node
    pub last_seen: Instant,
}

/// A k-bucket in the Kademlia routing table that stores up to k nodes.
///
/// K-buckets implement a least-recently seen cache of nodes in a given XOR distance range.
/// They are structured to provide resistance against certain DoS attacks by preferring
/// long-lived nodes over newer nodes when the bucket is full.
///
/// # Properties
/// - Stores up to k nodes (where k is a system-wide parameter)
/// - Implements a least-recently seen eviction policy
/// - Orders nodes based on last contact time
#[derive(Clone)]
pub struct KBucket {
    /// Queue of nodes in this bucket, ordered by time last seen
    nodes: VecDeque<NodeId>,
}

impl KBucket {
    /// Creates a new, empty k-bucket with capacity K.
    ///
    /// # Returns
    /// A new KBucket instance that can store up to K nodes.
    pub fn new() -> Self {
        KBucket {
            nodes: VecDeque::with_capacity(K),
        }
    }

    /// Updates the k-bucket with information about a node.
    ///
    /// This implementation follows the Kademlia paper's k-bucket update algorithm:
    /// 1. If the node already exists, move it to the tail (most-recently seen)
    /// 2. If the bucket isn't full, add the node to the tail
    /// 3. If the bucket is full, drop the new node (favoring existing nodes)
    ///
    /// # Arguments
    /// * `node` - The NodeId to update or insert
    ///
    /// # Note
    /// This implementation uses a simplified eviction policy. The original Kademlia
    /// paper suggests pinging the least-recently seen node and only evicting it if
    /// it fails to respond.
    pub fn update(&mut self, node: NodeId) {
        if let Some(pos) = self.nodes.iter().position(|n| *n == node) {
            self.nodes.remove(pos);
        }

        if self.nodes.len() < K {
            self.nodes.push_back(node);
        }
    }
}

/// The Kademlia routing table, consisting of k-buckets organized by XOR distance.
///
/// The routing table maintains k-buckets based on the XOR distance between the local
/// node's ID and other nodes' IDs. This structure allows efficient lookups with
/// logarithmic complexity.
///
/// # Properties
/// - Contains KEY_SIZE k-buckets (one for each bit of the node ID)
/// - Each bucket stores nodes with specific XOR distance properties
/// - Provides O(log N) lookup complexity for network size N
#[derive(Clone)]
pub struct RoutingTable {
    /// ID of the local node
    node_id: NodeId,
    /// Vector of k-buckets, indexed by the distance prefix length
    buckets: Vec<KBucket>,
}

impl RoutingTable {
    /// Creates a new routing table for the given local node ID.
    ///
    /// Initializes KEY_SIZE empty k-buckets, one for each bit position
    /// in the node ID space.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the local node
    pub fn new(node_id: NodeId) -> Self {
        let buckets = (0..KEY_SIZE).map(|_| KBucket::new()).collect();
        RoutingTable { node_id, buckets }
    }

    /// Updates the routing table with information about a node.
    ///
    /// # Arguments
    /// * `node` - The NodeId to update or insert
    ///
    /// # Implementation Details
    /// 1. Calculates the appropriate bucket index based on XOR distance
    /// 2. Updates the corresponding k-bucket
    pub fn update(&mut self, node: NodeId) {
        let bucket_index = self.bucket_index(&node);
        self.buckets[bucket_index].update(node);
    }

    /// Finds the closest nodes to a target ID in the routing table.
    ///
    /// # Arguments
    /// * `target` - The target NodeId to find close nodes to
    /// * `count` - Maximum number of nodes to return
    ///
    /// # Returns
    /// A vector of the closest NodeIds, sorted by XOR distance from the target
    ///
    /// # Implementation Details
    /// 1. Collects all nodes from all buckets
    /// 2. Sorts them by XOR distance to the target
    /// 3. Returns the closest `count` nodes
    pub fn closest_nodes(&self, target: &NodeId, count: usize) -> Vec<NodeId> {
        let mut nodes = Vec::new();

        for bucket in &self.buckets {
            nodes.extend(bucket.nodes.iter().cloned());
        }

        nodes.sort_by_key(|n| Distance::between(n, target));
        nodes.truncate(count);
        nodes
    }

    /// Calculates the index of the k-bucket for a given node ID.
    ///
    /// The bucket index is determined by the length of the common prefix
    /// (in bits) between the local node's ID and the given node ID.
    ///
    /// # Arguments
    /// * `node` - The NodeId to calculate the bucket index for
    ///
    /// # Returns
    /// The index of the appropriate k-bucket
    fn bucket_index(&self, node: &NodeId) -> usize {
        let distance = Distance::between(&self.node_id, node);
        distance.leading_zeros() as usize
    }
}
