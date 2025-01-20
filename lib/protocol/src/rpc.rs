// src/rpc.rs
// Implementation of RPC (Remote Procedure Call)
use crate::{Key, NodeId};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct RpcServer {
    socket: UdpSocket,
}

impl RpcServer {
    pub async fn new() -> Self {
        RpcServer {
            socket: UdpSocket::bind("0.0.0.0:0").await.unwrap(),
        }
    }
}

pub struct RpcClient {
    socket: UdpSocket,
}

impl RpcClient {
    pub async fn new() -> Self {
        RpcClient {
            socket: UdpSocket::bind("0.0.0.0:0").await.unwrap(),
        }
    }

    pub async fn ping(&self, addr: SocketAddr) -> bool {
        // Send a PING message and wait for a response
        todo!()
    }

    pub async fn store(&self, node: NodeId, key: Key, value: Vec<u8>) -> bool {
        // Send a STORE message
        todo!()
    }

    pub async fn find_node(&self, node: NodeId, target: Key) -> Vec<NodeId> {
        // Send a FIND_NODE message and wait for a response
        todo!()
    }

    pub async fn find_value(&self, node: NodeId, key: Key) -> Result<Vec<u8>, Vec<NodeId>> {
        // Send a FIND_VALUE message and wait for a response
        todo!()
    }
}
