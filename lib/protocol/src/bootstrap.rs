use std::path::PathBuf;
use tokio::net::unix::SocketAddr;
use crate::{Key, Node};

const STORAGE_PERSISTENCE_FILE: &str = "persistencies.cksv";
const DEFAULT_STORAGE_PATH: &str = concat!(env!("HOME"), "/.compute-dht");

#[tokio::main]
async fn bootstrap_node() -> anyhow::Result<()> {
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;
    let storage_path = PathBuf::from("./node_data");
    let mut node = Node::new(addr, storage_path).await?;

    // Save data
    let key = Key::random();  // Generate a random key
    let value = b"Hello, DHT!".to_vec();
    node.store(key, value).await?;

    // Start the server
    node.run().await?;

    Ok(())
}
