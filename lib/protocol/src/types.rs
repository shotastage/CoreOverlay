// src/types.rs
use crate::KEY_SIZE;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId([u8; KEY_SIZE / 8]);

impl NodeId {
    pub fn new(bytes: [u8; KEY_SIZE / 8]) -> Self {
        NodeId(bytes)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; KEY_SIZE / 8];
        rng.fill_bytes(&mut bytes);
        NodeId(bytes)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Distance([u8; KEY_SIZE / 8]);

impl Distance {
    pub fn between(a: &NodeId, b: &NodeId) -> Self {
        let mut result = [0u8; KEY_SIZE / 8];
        for i in 0..KEY_SIZE / 8 {
            result[i] = a.0[i] ^ b.0[i];
        }
        Distance(result)
    }

    pub fn leading_zeros(&self) -> u32 {
        for (i, &byte) in self.0.iter().enumerate() {
            if byte != 0 {
                return (i * 8) as u32 + byte.leading_zeros();
            }
        }
        (KEY_SIZE / 8 * 8) as u32
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implementation of the Key type
pub type Key = NodeId;
