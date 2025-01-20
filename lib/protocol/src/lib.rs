// src/lib.rs
// Main library file for Kademlia DHT
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

pub mod node;
pub mod routing;
pub mod rpc;
pub mod storage;
pub mod types;

pub use node::Node;
pub use routing::RoutingTable;
pub use types::{Distance, Key, NodeId};

// Constant definitions
pub const K: usize = 20; // Size of the k-bucket
pub const ALPHA: usize = 3; // Number of parallel lookups
pub const KEY_SIZE: usize = 160; // Key size in bits
pub const REPUBLISH_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
pub const BUCKET_REFRESH_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
