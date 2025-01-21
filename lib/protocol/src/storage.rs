//! Implementation of the local key-value storage system for Kademlia nodes.
//!
//! This module provides temporary storage for key-value pairs with automatic expiration.
//! Values are stored with a time-to-live (TTL) and are automatically removed after
//! expiration during cleanup operations.

use crate::Key;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// An entry in the storage system containing a value and its expiration time.
///
/// Each entry consists of:
/// - The actual value being stored
/// - A timestamp indicating when the entry should expire
struct StorageEntry {
    /// The stored value as a byte vector
    value: Vec<u8>,
    /// Timestamp when this entry should expire
    expiry: Instant,
}

/// Storage implementation for a Kademlia node.
///
/// The storage system provides:
/// - Temporary storage of key-value pairs
/// - Automatic expiration based on TTL
/// - Cleanup of expired entries
///
/// This implements the local storage component described in the Kademlia paper,
/// where nodes temporarily store values they're responsible for until expiration.
pub struct Storage {
    /// HashMap storing the key-value pairs with their expiration times
    data: HashMap<Key, StorageEntry>,
}

impl Storage {
    /// Creates a new empty storage instance.
    ///
    /// # Returns
    /// A new Storage instance with an empty HashMap
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
        }
    }

    /// Stores a value with the specified time-to-live.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The value to store
    /// * `ttl` - Duration after which the value should expire
    ///
    /// # Implementation Details
    /// - If the key already exists, the old value is replaced
    /// - The expiry time is calculated as the current time plus the TTL
    /// - Values are not automatically removed upon expiration; cleanup() must be called
    pub fn store(&mut self, key: Key, value: Vec<u8>, ttl: Duration) {
        let entry = StorageEntry {
            value,
            expiry: Instant::now() + ttl,
        };
        self.data.insert(key, entry);
    }

    /// Retrieves a value by its key if it exists and hasn't expired.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Option<&Vec<u8>>` - Some(value) if found and not expired, None otherwise
    ///
    /// # Implementation Note
    /// This method checks expiration time but does not remove expired entries.
    /// Call cleanup() periodically to remove expired entries.
    pub fn get(&self, key: &Key) -> Option<&Vec<u8>> {
        self.data
            .get(key)
            .filter(|entry| entry.expiry > Instant::now())
            .map(|entry| &entry.value)
    }

    /// Removes all expired entries from the storage.
    ///
    /// This method should be called periodically to prevent the storage
    /// from growing indefinitely with expired entries. The frequency of cleanup
    /// calls is a trade-off between memory usage and performance overhead.
    ///
    /// # Implementation Details
    /// - Uses HashMap's retain method to efficiently remove expired entries
    /// - Compares each entry's expiration time against the current time
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, entry| entry.expiry > now);
    }
}
