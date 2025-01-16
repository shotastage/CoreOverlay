use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

mod node;
mod routing;
mod rpc;
mod store;

pub use node::Node;
pub use routing::RoutingTable;
pub use rpc::{Rpc, RpcMessage};
pub use store::Store;

// NodeId は 160bit の ID を表現
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId([u8; 20]);

impl NodeId {
    pub fn distance(&self, other: &NodeId) -> NodeId {
        let mut result = [0u8; 20];
        for i in 0..20 {
            result[i] = self.0[i] ^ other.0[i];
        }
        NodeId(result)
    }
}
