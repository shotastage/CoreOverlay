use crate::{Key, NodeId};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct RpcServer {
    socket: UdpSocket,
}

pub struct RpcClient {
    socket: UdpSocket,
}

// Define the types of RPC messages
#[derive(Serialize, Deserialize)]
enum RpcMessage {
    Ping {
        sender: NodeId,
    },
    Store {
        sender: NodeId,
        key: Key,
        value: Vec<u8>,
    },
    FindNode {
        sender: NodeId,
        target: Key,
    },
    FindValue {
        sender: NodeId,
        key: Key,
    },
}

// Define the types of RPC responses
#[derive(Serialize, Deserialize)]
enum RpcResponse {
    Pong {
        responder: NodeId,
    },
    NodesFound {
        responder: NodeId,
        nodes: Vec<(NodeId, SocketAddr)>,
    },
    ValueFound {
        responder: NodeId,
        value: Vec<u8>,
    },
    Stored {
        responder: NodeId,
        success: bool,
    },
}

impl RpcServer {
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(RpcServer { socket })
    }

    pub async fn start(&self, node_id: NodeId) -> Result<()> {
        let mut buf = vec![0u8; 65536]; // Maximum UDP packet size

        loop {
            let (size, src) = self.socket.recv_from(&mut buf).await?;
            let message: RpcMessage = bincode::deserialize(&buf[..size])?;

            let response = match message {
                RpcMessage::Ping { sender } => RpcResponse::Pong { responder: node_id },
                RpcMessage::Store { sender, key, value } => {
                    // TODO: Implement actual storage logic
                    RpcResponse::Stored {
                        responder: node_id,
                        success: true,
                    }
                }
                RpcMessage::FindNode { sender, target } => {
                    // TODO: Implement node lookup logic
                    RpcResponse::NodesFound {
                        responder: node_id,
                        nodes: vec![], // Return closest nodes from routing table
                    }
                }
                RpcMessage::FindValue { sender, key } => {
                    // TODO: Implement value lookup logic
                    RpcResponse::NodesFound {
                        responder: node_id,
                        nodes: vec![], // Return closest nodes if value not found
                    }
                }
            };

            let response_bytes = bincode::serialize(&response)?;
            self.socket.send_to(&response_bytes, src).await?;
        }
    }
}

impl RpcClient {
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(RpcClient { socket })
    }

    pub async fn ping(&self, node: NodeId, addr: SocketAddr) -> Result<bool> {
        let message = RpcMessage::Ping { sender: node };

        let message_bytes = bincode::serialize(&message)?;
        self.socket.send_to(&message_bytes, addr).await?;

        let mut buf = vec![0u8; 65536];
        let (size, _) = self.socket.recv_from(&mut buf).await?;

        match bincode::deserialize(&buf[..size])? {
            RpcResponse::Pong { .. } => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn store(
        &self,
        node: NodeId,
        target: NodeId,
        addr: SocketAddr,
        key: Key,
        value: Vec<u8>,
    ) -> Result<bool> {
        let message = RpcMessage::Store {
            sender: node,
            key,
            value,
        };

        let message_bytes = bincode::serialize(&message)?;
        self.socket.send_to(&message_bytes, addr).await?;

        let mut buf = vec![0u8; 65536];
        let (size, _) = self.socket.recv_from(&mut buf).await?;

        match bincode::deserialize(&buf[..size])? {
            RpcResponse::Stored { success, .. } => Ok(success),
            _ => Ok(false),
        }
    }

    pub async fn find_node(
        &self,
        node: NodeId,
        target: Key,
        addr: SocketAddr,
    ) -> Result<Vec<(NodeId, SocketAddr)>> {
        let message = RpcMessage::FindNode {
            sender: node,
            target,
        };

        let message_bytes = bincode::serialize(&message)?;
        self.socket.send_to(&message_bytes, addr).await?;

        let mut buf = vec![0u8; 65536];
        let (size, _) = self.socket.recv_from(&mut buf).await?;

        match bincode::deserialize(&buf[..size])? {
            RpcResponse::NodesFound { nodes, .. } => Ok(nodes),
            _ => Ok(vec![]),
        }
    }

    pub async fn find_value(
        &self,
        node: NodeId,
        key: Key,
        addr: SocketAddr,
    ) -> Result<Result<Vec<u8>, Vec<(NodeId, SocketAddr)>>> {
        let message = RpcMessage::FindValue { sender: node, key };

        let message_bytes = bincode::serialize(&message)?;
        self.socket.send_to(&message_bytes, addr).await?;

        let mut buf = vec![0u8; 65536];
        let (size, _) = self.socket.recv_from(&mut buf).await?;

        match bincode::deserialize(&buf[..size])? {
            RpcResponse::ValueFound { value, .. } => Ok(Ok(value)),
            RpcResponse::NodesFound { nodes, .. } => Ok(Err(nodes)),
            _ => Ok(Err(vec![])),
        }
    }
}
