// src/routing.rs
// Implementation of the routing table
use crate::KEY_SIZE;
use crate::{Distance, NodeId, K};
use std::collections::VecDeque;

pub struct KBucket {
    nodes: VecDeque<NodeId>,
}

impl KBucket {
    pub fn new() -> Self {
        KBucket {
            nodes: VecDeque::with_capacity(K),
        }
    }

    pub fn update(&mut self, node: NodeId) {
        if let Some(pos) = self.nodes.iter().position(|n| *n == node) {
            self.nodes.remove(pos);
        }

        if self.nodes.len() < K {
            self.nodes.push_back(node);
        }
    }
}

pub struct RoutingTable {
    node_id: NodeId,
    buckets: Vec<KBucket>,
}

impl RoutingTable {
    pub fn new(node_id: NodeId) -> Self {
        let buckets = (0..KEY_SIZE).map(|_| KBucket::new()).collect();
        RoutingTable { node_id, buckets }
    }

    pub fn update(&mut self, node: NodeId) {
        let bucket_index = self.bucket_index(&node);
        self.buckets[bucket_index].update(node);
    }

    pub fn closest_nodes(&self, target: &NodeId, count: usize) -> Vec<NodeId> {
        let mut nodes = Vec::new();

        for bucket in &self.buckets {
            nodes.extend(bucket.nodes.iter().cloned());
        }

        nodes.sort_by_key(|n| Distance::between(n, target));
        nodes.truncate(count);
        nodes
    }

    fn bucket_index(&self, node: &NodeId) -> usize {
        let distance = Distance::between(&self.node_id, node);
        distance.leading_zeros() as usize
    }
}
