// Implement a simple key-value store with TTL (Time To Live) feature.

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Store {
    data: HashMap<Vec<u8>, (Vec<u8>, Instant)>,
    ttl: Duration,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
            ttl: Duration::from_secs(86400), // 24時間のTTL
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, (value, Instant::now()));
    }

    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.data.get(key).and_then(|(value, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(value)
            } else {
                None
            }
        })
    }

    pub fn cleanup(&mut self) {
        self.data.retain(|_, (_, timestamp)| timestamp.elapsed() < self.ttl);
    }
}
