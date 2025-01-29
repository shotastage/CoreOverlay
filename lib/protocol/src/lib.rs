//! A Rust implementation of the Kademlia Distributed Hash Table (DHT) protocol.
//!
//! # Overview
//! Kademlia is a peer-to-peer distributed hash table (DHT) designed for decentralized
//! key-value storage and retrieval. This implementation follows the original Kademlia
//! paper by Petar Maymounkov and David Mazières.
//!
//! # Key Features
//! - XOR-based metric for node/key distance calculation
//! - k-bucket routing tables for efficient node lookup
//! - Iterative node lookups with parallel queries
//! - Decentralized key-value storage
//!
//! # Architecture
//! The library is organized into several modules:
//! - `node`: Core node implementation and network operations
//! - `routing`: k-bucket routing table implementation
//! - `rpc`: Network communication protocol
//! - `storage`: Key-value data storage
//! - `types`: Core type definitions (NodeId, Key, Distance)
//!
//! # Example
//! ```rust,no_run
//! use kademlia::{Node, Key};
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() {
//!     let addr = "127.0.0.1:8000".parse().unwrap();
//!     let mut node = Node::new(addr).await.unwrap();
//!
//!     // Store a value
//!     let key = Key::random();
//!     let value = b"Hello, Kademlia!".to_vec();
//!     node.store(key, value).await.unwrap();
//! }
//! ```

use std::time::Duration;

pub mod node;
pub mod routing;
pub mod rpc;
pub mod storage;
pub mod types;
mod bootstrap;

pub use node::Node;
pub use routing::RoutingTable;
pub use types::{Distance, Key, NodeId};

/// The size of a k-bucket (k) in the Kademlia routing table.
///
/// This value represents the maximum number of nodes that can be stored in each k-bucket.
/// The Kademlia paper suggests k = 20 as a reasonable system-wide replication parameter
/// that is large enough to provide reliable operation while keeping message counts low.
pub const K: usize = 20;

/// The number of parallel lookups (α) to perform during node lookup operations.
///
/// This parameter controls the amount of lookup parallelization. The original Kademlia
/// paper suggests α = 3 as an optimal value that provides good lookup performance
/// while keeping bandwidth usage reasonable.
///
/// A larger α might speed up lookups but increases network traffic, while a smaller α
/// would reduce network usage but make lookups slower.
pub const ALPHA: usize = 3;

/// The size of node IDs and keys in bits.
///
/// Kademlia uses 160-bit node IDs and keys, matching the output size of SHA-1.
/// This provides:
/// - A large enough address space to make ID collisions extremely unlikely
/// - Consistent hash distribution for load balancing
/// - Compatibility with SHA-1 for key generation
pub const KEY_SIZE: usize = 160;

/// The interval at which values should be republished in the network.
///
/// To maintain data availability in the face of node churn, values are periodically
/// republished. This constant sets that interval to 1 hour, which:
/// - Ensures data persistence
/// - Helps maintain replica count in the face of node failures
/// - Keeps network overhead reasonable
pub const REPUBLISH_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour

/// The interval at which k-buckets should be refreshed.
///
/// Each k-bucket should be refreshed periodically to ensure the routing table
/// remains accurate and up-to-date. This is done by performing a node lookup
/// for a random key in the bucket's range. The 1-hour interval:
/// - Maintains routing table freshness
/// - Helps detect failed nodes
/// - Keeps routing information current
pub const BUCKET_REFRESH_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
