// src/storage.rs
// Implementation of data storage
use crate::Key;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct StorageEntry {
    value: Vec<u8>,
    expiry: Instant,
}

pub struct Storage {
    data: HashMap<Key, StorageEntry>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: Key, value: Vec<u8>, ttl: Duration) {
        let entry = StorageEntry {
            value,
            expiry: Instant::now() + ttl,
        };
        self.data.insert(key, entry);
    }

    pub fn get(&self, key: &Key) -> Option<&Vec<u8>> {
        self.data
            .get(key)
            .filter(|entry| entry.expiry > Instant::now())
            .map(|entry| &entry.value)
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, entry| entry.expiry > now);
    }
}
