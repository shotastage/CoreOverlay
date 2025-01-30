//! Core types for the Kademlia DHT implementation.
//!
//! This module defines the fundamental types used throughout the Kademlia system:
//! - NodeId: 160-bit identifiers for nodes
//! - Distance: XOR distance metric between NodeIds
//! - Key: Type alias for NodeId used in key-value storage
//!
//! The design follows the Kademlia paper's specifications for node identifiers
//! and the XOR metric space.

use crate::KEY_SIZE;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::cmp::Ordering;
use std::fmt;

/// A 160-bit identifier for nodes in the Kademlia network.
///
/// NodeIds are uniformly distributed in the 160-bit identifier space,
/// as specified in the Kademlia paper. This provides:
/// - Even distribution of nodes across the ID space
/// - Equal responsibility for key storage among nodes
/// - Resistance to certain attacks through randomization
#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId([u8; KEY_SIZE / 8]);

impl NodeId {
    /// Returns a reference to the underlying byte array.
    ///
    /// # Returns
    /// A slice containing the node ID's bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl NodeId {
    /// Creates a new NodeId from an existing byte array.
    ///
    /// # Arguments
    /// * `bytes` - A 20-byte array (160 bits) to use as the NodeId
    ///
    /// # Returns
    /// A new NodeId containing the provided bytes
    pub fn new(bytes: [u8; KEY_SIZE / 8]) -> Self {
        NodeId(bytes)
    }

    /// Generates a random NodeId using a cryptographically secure RNG.
    ///
    /// This implements the Kademlia requirement for uniform distribution
    /// of node IDs across the identifier space.
    ///
    /// # Returns
    /// A new randomly generated NodeId
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; KEY_SIZE / 8];
        rng.fill_bytes(&mut bytes);
        NodeId(bytes)
    }
}

/// Represents the XOR distance between two NodeIds in the Kademlia metric space.
///
/// The Distance type implements Kademlia's XOR metric which has these properties:
/// 1. d(x,x) = 0           (identity)
/// 2. d(x,y) > 0, if x ≠ y (non-zero)
/// 3. d(x,y) = d(y,x)      (symmetry)
/// 4. d(x,z) ≤ d(x,y) + d(y,z) (triangle inequality)
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Distance([u8; KEY_SIZE / 8]);

impl Distance {
    /// Calculates the XOR distance between two NodeIds.
    ///
    /// This implements the core Kademlia distance metric: d(x,y) = x ⊕ y
    ///
    /// # Arguments
    /// * `a` - First NodeId
    /// * `b` - Second NodeId
    ///
    /// # Returns
    /// The XOR distance between the two NodeIds
    pub fn between(a: &NodeId, b: &NodeId) -> Self {
        let mut result = [0u8; KEY_SIZE / 8];
        for i in 0..KEY_SIZE / 8 {
            result[i] = a.0[i] ^ b.0[i];
        }
        Distance(result)
    }

    /// Counts the number of leading zero bits in the distance.
    ///
    /// This is used to determine which k-bucket a node belongs in,
    /// based on the most significant bit of difference between NodeIds.
    ///
    /// # Returns
    /// The number of leading zero bits in the distance
    pub fn leading_zeros(&self) -> u32 {
        for (i, &byte) in self.0.iter().enumerate() {
            if byte != 0 {
                return (i * 8) as u32 + byte.leading_zeros();
            }
        }
        (KEY_SIZE / 8 * 8) as u32
    }

    /// Returns a reference to the underlying byte array.
    ///
    /// # Returns
    /// A slice containing the distance value's bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Implementation of total ordering for Distance based on XOR metric.
///
/// This allows distances to be compared and sorted, which is essential
/// for finding the k closest nodes to a target ID.
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

/// Implementation of partial ordering for Distance.
///
/// Delegates to the total ordering implementation since XOR distances
/// are always comparable.
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Type alias for NodeId used as keys in the DHT.
///
/// In Kademlia, keys are drawn from the same 160-bit space as NodeIds.
/// This allows the XOR metric to be used for routing to stored values.
pub type Key = NodeId;
