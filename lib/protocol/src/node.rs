// src/node.rs
// Implementation of a Kademlia node
use crate::rpc::{RpcClient, RpcServer};
use crate::storage::Storage;
use crate::types::Distance;
use crate::{Key, NodeId, RoutingTable, ALPHA, K};
use std::collections::HashSet;
use std::net::SocketAddr;

pub struct Node {
    id: NodeId,
    addr: SocketAddr,
    routing_table: RoutingTable,
    storage: Storage,
    rpc_server: RpcServer,
    rpc_client: RpcClient,
}

impl Node {
    pub async fn new(addr: SocketAddr) -> Self {
        let id = NodeId::random();
        let routing_table = RoutingTable::new(id);
        let storage = Storage::new();
        let rpc_server = RpcServer::new().await;
        let rpc_client = RpcClient::new().await;

        Node {
            id,
            addr,
            routing_table,
            storage,
            rpc_server,
            rpc_client,
        }
    }

    pub async fn store(&mut self, key: Key, value: Vec<u8>) {
        // Find nodes to store the value
        let nodes = self.lookup_nodes(key).await;
        for node in nodes {
            self.rpc_client.store(node, key, value.clone()).await;
        }
    }

    pub async fn lookup_nodes(&self, key: Key) -> Vec<NodeId> {
        let mut closest = self.routing_table.closest_nodes(&key, K);
        let mut contacted = HashSet::new();
        let mut pending = HashSet::new();

        while !closest.is_empty() {
            let mut concurrent_lookups = Vec::new();

            // Query up to ALPHA nodes in parallel
            for node_id in closest.iter().take(ALPHA) {
                if !contacted.contains(node_id) && !pending.contains(node_id) {
                    pending.insert(*node_id);
                    let lookup = self.rpc_client.find_node(*node_id, key);
                    concurrent_lookups.push(lookup);
                }
            }

            // Wait for responses
            let responses = futures::future::join_all(concurrent_lookups).await;

            // Process responses
            for new_nodes in responses {
                // Process results from find_node
                closest.extend(new_nodes);
            }

            // Sort results by distance and limit to K nodes
            closest.sort_by_key(|n| Distance::between(n, &key));
            closest.truncate(K);

            // Update contacted and pending sets
            for node_id in closest.iter().take(ALPHA) {
                pending.remove(node_id);
                contacted.insert(*node_id);
            }
        }

        closest
    }
}
