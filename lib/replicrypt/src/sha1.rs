//! A module providing SHA-1 hash calculation functionality.
//!
//! This module offers a simple interface for computing SHA-1 hashes of arbitrary data
//! that implements the `AsRef<[u8]>` trait.

use sha1::{Digest, Sha1};

/// Calculates the SHA-1 hash of the provided data and returns it as a hexadecimal string.
///
/// This function accepts any type that can be referenced as a byte slice (i.e., implements
/// `AsRef<[u8]>`). This includes strings, byte arrays, vectors, and more.
///
/// # Arguments
///
/// * `data` - The data to hash, which must implement `AsRef<[u8]>`
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the SHA-1 hash.
///
/// # Examples
///
/// ```
/// use replicrypt::calculate_sha1;
///
/// let hash = calculate_sha1("Hello, World!");
/// assert_eq!(hash, "907d14fb3af2b0d4f18c2d46abe8aedce17367bd");
///
/// // Works with different types that implement AsRef<[u8]>
/// let bytes = vec![1, 2, 3, 4, 5];
/// let hash = calculate_sha1(bytes);
/// ```
pub fn calculate_sha1<T: AsRef<[u8]>>(data: T) -> String {
    // Create an SHA1 hasher
    let mut hasher = Sha1::new();

    // Input data into the hasher
    hasher.update(data);

    // Compute the hash value and return it as a hexadecimal string
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test SHA-1 hash of an empty string
    #[test]
    fn test_empty_string() {
        let empty = "";
        // SHA1("") = da39a3ee5e6b4b0d3255bfef95601890afd80709
        assert_eq!(
            calculate_sha1(empty),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
    }

    /// Test SHA-1 hash of a basic ASCII string
    #[test]
    fn test_basic_string() {
        let text = "Hello, World!";
        // SHA1("Hello, World!") = 907d14fb3af2b0d4f18c2d46abe8aedce17367bd
        assert_eq!(
            calculate_sha1(text),
            "907d14fb3af2b0d4f18c2d46abe8aedce17367bd"
        );
    }

    /// Test SHA-1 hash of a String type
    #[test]
    fn test_string_type() {
        let text = String::from("Hello, World!");
        // Verify that a `String` type produces the same SHA1 hash
        assert_eq!(
            calculate_sha1(text),
            "907d14fb3af2b0d4f18c2d46abe8aedce17367bd"
        );
    }

    /// Test SHA-1 hash of a byte vector
    #[test]
    fn test_byte_array() {
        let bytes = vec![1, 2, 3, 4, 5];
        // Verify SHA1 hash for a vector of bytes
        assert_eq!(
            calculate_sha1(bytes),
            "11966ab9c099f8fabf74b292cf4b1e0d55974165"
        );
    }

    /// Test SHA-1 hash of a Unicode string
    #[test]
    fn test_unicode_string() {
        let text = "こんにちは、世界！";
        // SHA1("こんにちは、世界！") = 8b38efdabdac695c2a2f47e6eb3a2e4a19cd0628
        assert_eq!(
            calculate_sha1(text),
            "8b38efdabdac695c2a2f47e6eb3a2e4a19cd0628"
        );
    }

    /// Test SHA-1 hash of a long string
    #[test]
    fn test_long_string() {
        let text = "a".repeat(1000);
        // SHA1 of a string with 1000 'a' characters
        assert_eq!(
            calculate_sha1(text),
            "c5f289104b54a6cba1d708f93b3ac48ff774b86a"
        );
    }

    /// Test SHA-1 hash of a byte slice
    #[test]
    fn test_slice_bytes() {
        let bytes = [1, 2, 3, 4, 5];
        // Verify SHA1 hash for a byte slice
        assert_eq!(
            calculate_sha1(&bytes[..]),
            "11966ab9c099f8fabf74b292cf4b1e0d55974165"
        );
    }

    /// Test that different inputs produce different hashes
    #[test]
    fn test_different_inputs_produce_different_hashes() {
        let hash1 = calculate_sha1("Hello");
        let hash2 = calculate_sha1("Hello!");
        // Ensure that different inputs produce different SHA1 hashes
        assert_ne!(hash1, hash2);
    }

    /// Test that identical inputs produce identical hashes
    #[test]
    fn test_same_inputs_produce_same_hashes() {
        let hash1 = calculate_sha1("Hello, World!");
        let hash2 = calculate_sha1("Hello, World!");
        // Ensure that identical inputs produce the same SHA1 hash
        assert_eq!(hash1, hash2);
    }
}
