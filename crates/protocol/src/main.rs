//! A module implementing a Kademlia DHT bootstrap node.
//!
//! This module provides initialization and bootstrap functionality
//! required for joining a Kademlia network.

use protocol::bootstrap_node;
use std::thread;
use std::time::Duration;

/// Displays a boot splash screen with Kademlia ASCII art logo.
///
/// The ASCII art is displayed line by line with a 200ms delay between each line,
/// providing visual feedback during application startup.
///
/// # Example
///
/// ```
/// boot_splash(); // Displays the ASCII art logo
/// ```
fn boot_splash() {
    let kademlia_art = [
        "*  *_   *    *___   _____ **  ** *     *__   _    ",
        "|/ |/ /  / \\  |  * \\ | *___|  \\/  | |   |_ _| / \\   ",
        "|  '  |  / * \\ | | | ||  *| | |\\/| | |    | | / _ \\  ",
        "| . \\ | / ___ \\| |_| || |___| |  | | |___ | |/ ___ \\ ",
        "|_|\\_\\/_/   \\_\\____/ |_____|_|  |_|_____|___/_/   \\_\\",
    ];

    for line in kademlia_art.iter() {
        println!("{}", line);
        thread::sleep(Duration::from_millis(200));
    }
}

/// Main entry point for the Kademlia DHT node.
///
/// Performs the following operations:
/// - Displays the boot splash
/// - Initializes and starts the node
/// - Runs the node in an infinite loop
///
/// The node continues running until it receives a Ctrl+C signal.
///
/// # Panics
///
/// May panic if the bootstrap process fails.
///
/// # Note
///
/// Bootstrap functionality is not implemented in the current version.
///
#[tokio::main]
async fn main() {
    boot_splash();
    println!("Welcome to the Kademlia DHT!");
    println!("Starting node...");
    bootstrap_node().await.unwrap();
    println!("Node started successfully");
    println!("Press Ctrl+C to stop the node");
    loop {
        thread::park();
    }
}
