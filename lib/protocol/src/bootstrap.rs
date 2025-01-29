use std::net::SocketAddr;
use std::path::PathBuf;
use anyhow::Result;
use crate::{Key, Node};
use std::{thread, time::Duration};

// Constants for storage configuration
const STORAGE_PERSISTENCE_FILE: &str = "persistencies.cksv";
const DEFAULT_STORAGE_PATH: &str = "./.compute-dht";

/// Bootstrap a new Kademlia DHT node
///
/// This function:
/// 1. Creates a new node with the specified address and storage path
/// 2. Stores some initial data in the DHT
/// 3. Starts the node's RPC server
///
/// # Returns
/// * `Result<()>` - Success or error
pub async fn bootstrap_node() -> Result<()> {
    // Create the storage directory if it doesn't exist
    let storage_path = PathBuf::from(DEFAULT_STORAGE_PATH);
    std::fs::create_dir_all(&storage_path)?;

    // Parse the socket address
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;

    // Initialize the node
    let mut node = Node::new(addr, storage_path).await?;

    // Store some initial data
    let key = Key::random();
    let value = b"Hello, DHT!".to_vec();
    node.store(key, value).await?;

    // Start the RPC server
    println!("Starting Kademlia DHT node on {}", addr);
    node.run().await?;

    Ok(())
}
