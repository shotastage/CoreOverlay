// Implementation of Kademlia routing table

use crate::NodeId;
use std::collections::VecDeque;
use std::net::SocketAddr;

const K: usize = 20; // k-bucketのサイズ

pub struct KBucket {
    nodes: VecDeque<(NodeId, SocketAddr)>,
}

impl KBucket {
    fn new() -> Self {
        KBucket {
            nodes: VecDeque::with_capacity(K),
        }
    }

    fn update(&mut self, id: NodeId, addr: SocketAddr) {
        // ノードの更新ロジックを実装
        if let Some(pos) = self.nodes.iter().position(|(nid, _)| *nid == id) {
            self.nodes.remove(pos);
        }

        if self.nodes.len() >= K {
            // k-bucketが満杯の場合の処理
            // 最も古いノードをping して、応答がなければ削除
            todo!()
        }

        self.nodes.push_back((id, addr));
    }
}

pub struct RoutingTable {
    node_id: NodeId,
    buckets: Vec<KBucket>,
}

impl RoutingTable {
    pub fn new(node_id: NodeId) -> Self {
        RoutingTable {
            node_id,
            buckets: (0..160).map(|_| KBucket::new()).collect(),
        }
    }

    pub fn update(&mut self, id: NodeId, addr: SocketAddr) {
        let bucket_idx = self.bucket_index(&id);
        self.buckets[bucket_idx].update(id, addr);
    }

    fn bucket_index(&self, id: &NodeId) -> usize {
        // 適切なk-bucketのインデックスを計算
        let distance = self.node_id.distance(id);
        // 最初の1ビットの位置を見つける
        for i in 0..160 {
            let byte = i / 8;
            let bit = 7 - (i % 8);
            if (distance.0[byte] & (1 << bit)) != 0 {
                return i;
            }
        }
        159
    }
}
