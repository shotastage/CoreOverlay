// Implementation of Kademlia node

use crate::{NodeId, RoutingTable, Rpc, Store};
use std::net::SocketAddr;

pub struct Node {
    id: NodeId,
    addr: SocketAddr,
    routing_table: RoutingTable,
    store: Store,
    rpc: Rpc,
}

impl Node {
    pub fn new(id: NodeId, addr: SocketAddr) -> Self {
        Node {
            id,
            addr,
            routing_table: RoutingTable::new(id),
            store: Store::new(),
            rpc: Rpc::new(),
        }
    }

    pub async fn bootstrap(&mut self, known_node: SocketAddr) {
        todo!()
    }

    pub async fn store(&mut self, key: Vec<u8>, value: Vec<u8>) {
        todo!()
    }

    pub async fn find_value(&mut self, key: Vec<u8>) -> Option<Vec<u8>> {
        todo!()
    }

    pub async fn find_node(&mut self, id: NodeId) -> Vec<(NodeId, SocketAddr)> {
        todo!()
    }
}
