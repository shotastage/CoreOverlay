use crate::Key;
use anyhow::Result;
use sled::Db;
use std::path::Path;
use std::time::{Duration, Instant};

/// Persistent storage implementation for a Kademlia node using Sled.
#[derive(Clone)]
pub struct Storage {
    /// Sled database instance
    db: Db,
}

impl Storage {
    /// Creates a new storage instance with Sled backend.
    ///
    /// # Arguments
    /// * `path` - Path where the database files will be stored
    ///
    /// # Returns
    /// A new Storage instance or an error if database creation fails
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::Config::new()
            .path(path)
            .mode(sled::Mode::HighThroughput) // Optimize for DHT workload
            .cache_capacity(1024 * 1024 * 64) // 64MB cache
            .open()?;

        Ok(Storage { db })
    }

    /// Stores a value with the specified time-to-live.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The value to store
    /// * `ttl` - Duration after which the value should expire
    pub fn store(&self, key: Key, mut value: Vec<u8>, ttl: Duration) -> Result<()> {
        // Prepend expiry timestamp to value
        let expiry = Instant::now() + ttl;
        let expiry_bytes = expiry
            .duration_since(Instant::now())
            .as_secs()
            .to_be_bytes();
        let mut stored_value = expiry_bytes.to_vec();
        stored_value.extend(value);

        self.db.insert(key.as_bytes(), stored_value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Retrieves a value by its key if it exists and hasn't expired.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Result<Option<Vec<u8>>>` - The value if found and not expired
    pub fn get(&self, key: &Key) -> Result<Option<Vec<u8>>> {
        if let Some(ivec) = self.db.get(key.as_bytes())? {
            let value = ivec.to_vec();

            // First 8 bytes are expiry timestamp
            if value.len() < 8 {
                return Ok(None);
            }

            let mut expiry_bytes = [0u8; 8];
            expiry_bytes.copy_from_slice(&value[..8]);
            let expiry_secs = u64::from_be_bytes(expiry_bytes);

            if Duration::from_secs(expiry_secs) > Instant::now().duration_since(Instant::now()) {
                Ok(Some(value[8..].to_vec()))
            } else {
                // Value has expired, remove it
                self.db.remove(key.as_bytes())?;
                self.db.flush()?;
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Performs cleanup of expired entries.
    ///
    /// Iterates through all entries and removes expired ones.
    /// This operation can be expensive for large datasets.
    pub fn cleanup(&self) -> Result<()> {
        let mut batch = sled::Batch::default();

        for item in self.db.iter() {
            let (key, value) = item?;
            if value.len() >= 8 {
                let mut expiry_bytes = [0u8; 8];
                expiry_bytes.copy_from_slice(&value[..8]);
                let expiry_secs = u64::from_be_bytes(expiry_bytes);

                if Duration::from_secs(expiry_secs) <= Instant::now().duration_since(Instant::now())
                {
                    batch.remove(key);
                }
            }
        }

        self.db.apply_batch(batch)?;
        self.db.flush()?;
        Ok(())
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        // Ensure all data is flushed to disk
        let _ = self.db.flush();
    }
}
