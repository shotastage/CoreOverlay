//! Implementation of the Kademlia RPC protocol.
//!
//! This module implements the four core Kademlia RPCs:
//! - PING: Probes a node to see if it is online
//! - STORE: Instructs a node to store a key-value pair
//! - FIND_NODE: Finds the k closest nodes to a given ID
//! - FIND_VALUE: Similar to FIND_NODE but returns a value if found

use crate::{Key, NodeId};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

/// Server component for handling incoming Kademlia RPC requests
pub struct RpcServer {
    /// UDP socket for receiving RPC messages
    socket: UdpSocket,
}

/// Client component for making outgoing Kademlia RPC requests
pub struct RpcClient {
    /// UDP socket for sending RPC messages
    socket: UdpSocket,
}

/// Enumeration of possible RPC message types in the Kademlia protocol.
///
/// Each variant contains the sender's NodeId and any additional data
/// required for that specific RPC type.
#[derive(Serialize, Deserialize)]
enum RpcMessage {
    /// Simple ping message to check if a node is alive
    Ping {
        /// ID of the sending node
        sender: NodeId,
    },
    /// Request to store a key-value pair
    Store {
        /// ID of the sending node
        sender: NodeId,
        /// Key under which to store the value
        key: Key,
        /// Value to be stored
        value: Vec<u8>,
    },
    /// Request to find the k closest nodes to a target
    FindNode {
        /// ID of the sending node
        sender: NodeId,
        /// Target ID to find close nodes to
        target: Key,
    },
    /// Request to find a value by its key
    FindValue {
        /// ID of the sending node
        sender: NodeId,
        /// Key of the value to find
        key: Key,
    },
}

/// Enumeration of possible RPC response types in the Kademlia protocol.
#[derive(Serialize, Deserialize)]
enum RpcResponse {
    /// Response to a Ping message
    Pong {
        /// ID of the responding node
        responder: NodeId,
    },
    /// Response containing k closest nodes to a target
    NodesFound {
        /// ID of the responding node
        responder: NodeId,
        /// Vector of found nodes and their addresses
        nodes: Vec<(NodeId, SocketAddr)>,
    },
    /// Response containing a requested value
    ValueFound {
        /// ID of the responding node
        responder: NodeId,
        /// The found value
        value: Vec<u8>,
    },
    /// Response confirming value storage
    Stored {
        /// ID of the responding node
        responder: NodeId,
        /// Whether the store operation succeeded
        success: bool,
    },
}

impl RpcServer {
    /// Creates a new RPC server bound to an arbitrary port.
    ///
    /// # Returns
    /// * `Result<Self>` - New RpcServer instance or error
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(RpcServer { socket })
    }

    /// Starts the RPC server's main loop handling incoming requests.
    ///
    /// # Arguments
    /// * `node_id` - The ID of this node for responses
    ///
    /// # Implementation Details
    /// - Uses a 64KB buffer for UDP packets
    /// - Deserializes incoming messages using bincode
    /// - Handles all four Kademlia RPC types
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
    /// Creates a new RPC client bound to an arbitrary port.
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(RpcClient { socket })
    }

    /// Sends a PING RPC to check if a node is alive.
    ///
    /// # Arguments
    /// * `node` - ID of the sending node
    /// * `addr` - Address to send the ping to
    ///
    /// # Returns
    /// * `Result<bool>` - true if PONG received, false otherwise
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

    /// Sends a STORE RPC to store a key-value pair on a node.
    ///
    /// # Arguments
    /// * `node` - ID of the sending node
    /// * `target` - ID of the target node
    /// * `addr` - Address of the target node
    /// * `key` - Key under which to store the value
    /// * `value` - Value to store
    ///
    /// # Returns
    /// * `Result<bool>` - Whether the store operation succeeded
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

    /// Sends a FIND_NODE RPC to find the k closest nodes to a target.
    ///
    /// # Arguments
    /// * `node` - ID of the sending node
    /// * `target` - Target ID to find close nodes to
    /// * `addr` - Address to send the request to
    ///
    /// # Returns
    /// * `Result<Vec<(NodeId, SocketAddr)>>` - Vector of found nodes and their addresses
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

    /// Sends a FIND_VALUE RPC to retrieve a stored value.
    ///
    /// # Arguments
    /// * `node` - ID of the sending node
    /// * `key` - Key of the value to find
    /// * `addr` - Address to send the request to
    ///
    /// # Returns
    /// * `Result<Result<Vec<u8>, Vec<(NodeId, SocketAddr)>>>` - Either:
    ///   - Ok(Vec<u8>) - The found value
    ///   - Err(Vec<(NodeId, SocketAddr)>) - k closest nodes if value not found
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
