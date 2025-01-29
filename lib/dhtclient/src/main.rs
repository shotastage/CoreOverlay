use anyhow::Result;
use protocol::{Key, NodeId, RpcClient};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::timeout;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dhtclient")]
#[command(about = "A client tool for Kademlia DHT")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// DHT node address in the format IP:PORT
    #[arg(short, long, default_value = "127.0.0.1:8000")]
    node: String,

    /// Timeout in seconds for operations
    #[arg(short, long, default_value = "5")]
    timeout: u64,
}

#[derive(Subcommand)]
enum Commands {
    /// List all key-value pairs
    List {
        /// Optional filter pattern
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Get a value by key
    Get {
        /// Key to lookup (in hex format)
        key: String,
    },

    /// Store a key-value pair
    Put {
        /// Key to store (in hex format)
        key: String,
        /// Value to store
        value: String,
    },

    /// Delete a key-value pair
    Delete {
        /// Key to delete (in hex format)
        key: String,
    },

    /// Show information about the DHT node
    Info,
}

/// Create a Key from a hex string
fn parse_key(hex_key: &str) -> Result<Key> {
    let key_bytes = hex::decode(hex_key)?;
    if key_bytes.len() != 20 {
        anyhow::bail!("Invalid key length. Expected 20 bytes (40 hex characters)");
    }
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&key_bytes);
    Ok(Key::new(bytes))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();

    // Parse node address
    let addr: SocketAddr = args.node.parse()?;
    let timeout_duration = Duration::from_secs(args.timeout);

    // Create RPC client
    let client = RpcClient::new().await?;
    let node_id = NodeId::random();

    match args.command {
        Commands::List { pattern } => {
            println!("Listing key-value pairs...");
            let result = timeout(
                timeout_duration,
                client.find_node(node_id, Key::random(), addr)
            ).await??;

            for (node_id, node_addr) in result {
                if let Ok(Ok(value)) = client.find_value(node_id, node_id, node_addr).await {
                    let key_hex = hex::encode(node_id.as_bytes());
                    let value_str = String::from_utf8_lossy(&value);

                    // Apply pattern filter if specified
                    if let Some(ref pattern) = pattern {
                        if !key_hex.contains(pattern) && !value_str.contains(pattern) {
                            continue;
                        }
                    }

                    println!("Key: {}", key_hex);
                    println!("Value: {}", value_str);
                    println!("---");
                }
            }
        },

        Commands::Get { key } => {
            let key = parse_key(&key)?;
            match timeout(
                timeout_duration,
                client.find_value(node_id, key, addr)
            ).await?? {
                Ok(value) => {
                    println!("Value: {}", String::from_utf8_lossy(&value));
                },
                Err(_) => {
                    println!("Key not found");
                }
            }
        },

        Commands::Put { key, value } => {
            let key = parse_key(&key)?;
            let success = timeout(
                timeout_duration,
                client.store(node_id, node_id, addr, key, value.into_bytes())
            ).await??;

            if success {
                println!("Value stored successfully");
            } else {
                println!("Failed to store value");
            }
        },

        Commands::Delete { key } => {
            let key = parse_key(&key)?;
            let success = timeout(
                timeout_duration,
                client.store(node_id, node_id, addr, key, Vec::new())  // Empty value for deletion
            ).await??;

            if success {
                println!("Key deleted successfully");
            } else {
                println!("Failed to delete key");
            }
        },

        Commands::Info => {
            // Try to ping the node
            let ping_result = timeout(
                timeout_duration,
                client.ping(node_id, addr)
            ).await??;

            println!("DHT Node Information");
            println!("-------------------");
            println!("Address: {}", addr);
            println!("Status: {}", if ping_result { "Online" } else { "Offline" });

            // Get routing table information
            let nodes = timeout(
                timeout_duration,
                client.find_node(node_id, Key::random(), addr)
            ).await??;

            println!("Known nodes: {}", nodes.len());
        },
    }

    Ok(())
}
