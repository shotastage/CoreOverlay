use std::{net::SocketAddr, time::SystemTime};

/// Size of K-buckets
const K: usize = 20;

/// NodeID length
const ID_LENGTH: usize = 160;

/// Represents a node in the network with an ID, address, and last seen time.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    id: NodeId,
    addr: SocketAddr,
    last_seen: SystemTime,
}

/// Represents a Node ID consisting of 160 bits (20 bytes).
#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeId([u8; 20]);

impl NodeId {
    /// Calculates the distance (XOR distance) between two Node IDs.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another NodeId to calculate the distance to.
    ///
    /// # Returns
    ///
    /// A new NodeId representing the XOR distance between the two Node IDs.
    fn distance(&self, other: &NodeId) -> NodeId {
        let mut result = [0u8; 20];
        for i in 0..20 {
            result[i] = self.0[i] ^ other.0[i];
        }
        NodeId(result)
    }
}
