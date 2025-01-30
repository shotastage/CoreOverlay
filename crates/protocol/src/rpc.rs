//! Implementation of the Kademlia RPC protocol.
//!
//! This module implements the four core Kademlia RPCs:
//! - PING: Probes a node to see if it is online
//! - STORE: Instructs a node to store a key-value pair
//! - FIND_NODE: Finds the k closest nodes to a given ID
//! - FIND_VALUE: Similar to FIND_NODE but returns a value if found

use crate::routing::RoutingTable;
use crate::storage::Storage;
use crate::{Key, NodeId, K};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

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

    /// Handles STORE RPC requests
    async fn handle_store(
        &self,
        node_id: NodeId,
        sender: NodeId,
        key: Key,
        value: Vec<u8>,
        storage: &Storage,
        routing_table: &mut RoutingTable,
    ) -> RpcResponse {
        // Update routing table with sender information
        routing_table.update(sender);

        // Store with 24 hour TTL
        let ttl = Duration::from_secs(24 * 60 * 60);
        match storage.store(key, value, ttl) {
            Ok(()) => RpcResponse::Stored {
                responder: node_id,
                success: true,
            },
            Err(_) => RpcResponse::Stored {
                responder: node_id,
                success: false,
            },
        }
    }

    /// Handles FIND_NODE RPC requests
    async fn handle_find_node(
        &self,
        node_id: NodeId,
        sender: NodeId,
        target: Key,
        routing_table: &mut RoutingTable,
    ) -> RpcResponse {
        // Update routing table with sender information
        routing_table.update(sender);

        // Find k closest nodes to target
        let closest_nodes = routing_table.closest_nodes(&target, K);

        // Convert NodeIds to (NodeId, SocketAddr) pairs
        // TODO: Get actual addresses from routing table
        let nodes: Vec<(NodeId, SocketAddr)> = closest_nodes
            .into_iter()
            .map(|id| (id, SocketAddr::from(([127, 0, 0, 1], 8000))))
            .collect();

        RpcResponse::NodesFound {
            responder: node_id,
            nodes,
        }
    }

    /// Handles FIND_VALUE RPC requests
    async fn handle_find_value(
        &self,
        node_id: NodeId,
        sender: NodeId,
        key: Key,
        storage: &Storage,
        routing_table: &mut RoutingTable,
    ) -> RpcResponse {
        // Update routing table with sender information
        routing_table.update(sender);

        // First try to find the value locally
        match storage.get(&key) {
            Ok(Some(value)) => RpcResponse::ValueFound {
                responder: node_id,
                value,
            },
            Ok(None) | Err(_) => {
                // If value not found, return k closest nodes
                let closest_nodes = routing_table.closest_nodes(&key, K);
                let nodes: Vec<(NodeId, SocketAddr)> = closest_nodes
                    .into_iter()
                    .map(|id| (id, SocketAddr::from(([127, 0, 0, 1], 8000))))
                    .collect();

                RpcResponse::NodesFound {
                    responder: node_id,
                    nodes,
                }
            }
        }
    }

    /// Starts the RPC server's main loop handling incoming requests.
    pub async fn start(
        &self,
        node_id: NodeId,
        storage: Storage,
        routing_table: Arc<Mutex<RoutingTable>>,
    ) -> Result<()> {
        let mut buf = vec![0u8; 65536]; // Maximum UDP packet size

        loop {
            let (size, src) = self.socket.recv_from(&mut buf).await?;
            let message: RpcMessage = bincode::deserialize(&buf[..size])?;

            // Clone Arc and get mutex lock
            let mut routing_table = routing_table.lock().await;

            let response = match message {
                RpcMessage::Ping { sender } => {
                    // Update routing table and respond with Pong
                    routing_table.update(sender);
                    RpcResponse::Pong { responder: node_id }
                }
                RpcMessage::Store { sender, key, value } => {
                    self.handle_store(node_id, sender, key, value, &storage, &mut routing_table)
                        .await
                }
                RpcMessage::FindNode { sender, target } => {
                    self.handle_find_node(node_id, sender, target, &mut routing_table)
                        .await
                }
                RpcMessage::FindValue { sender, key } => {
                    self.handle_find_value(node_id, sender, key, &storage, &mut routing_table)
                        .await
                }
            };

            // Drop routing table lock
            drop(routing_table);

            // Send response
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
