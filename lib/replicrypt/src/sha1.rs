use sha1::{Sha1, Digest};

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

    #[test]
    fn test_empty_string() {
        let empty = "";
        // SHA1("") = da39a3ee5e6b4b0d3255bfef95601890afd80709
        assert_eq!(
            calculate_sha1(empty),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
    }

    #[test]
    fn test_basic_string() {
        let text = "Hello, World!";
        // SHA1("Hello, World!") = 0a0a9f2a6772942557ab5355d76af442f8f65e01
        assert_eq!(
            calculate_sha1(text),
            "0a0a9f2a6772942557ab5355d76af442f8f65e01"
        );
    }

    #[test]
    fn test_string_type() {
        let text = String::from("Hello, World!");
        // Verify that a `String` type produces the same SHA1 hash
        assert_eq!(
            calculate_sha1(text),
            "0a0a9f2a6772942557ab5355d76af442f8f65e01"
        );
    }

    #[test]
    fn test_byte_array() {
        let bytes = vec![1, 2, 3, 4, 5];
        // Verify SHA1 hash for a vector of bytes
        assert_eq!(
            calculate_sha1(bytes),
            "11966ab9c099f8fabf74b292cf4b1e0d55974165"
        );
    }

    #[test]
    fn test_unicode_string() {
        let text = "こんにちは、世界！";
        // Verify SHA1 hash for a Unicode string
        assert_eq!(
            calculate_sha1(text),
            "89b9b54481c896e5cd35b4dd8e5a176f898407e0"
        );
    }

    #[test]
    fn test_long_string() {
        let text = "a".repeat(1000);
        // Verify SHA1 hash for a long string (example hash value)
        assert_eq!(
            calculate_sha1(text),
            "29b1c3065c2c2a281214burnot62c98e67baba9c"  // This value is for demonstration purposes
        );
    }

    #[test]
    fn test_slice_bytes() {
        let bytes = [1, 2, 3, 4, 5];
        // Verify SHA1 hash for a byte slice
        assert_eq!(
            calculate_sha1(&bytes[..]),
            "11966ab9c099f8fabf74b292cf4b1e0d55974165"
        );
    }

    #[test]
    fn test_different_inputs_produce_different_hashes() {
        let hash1 = calculate_sha1("Hello");
        let hash2 = calculate_sha1("Hello!");
        // Ensure that different inputs produce different SHA1 hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_inputs_produce_same_hashes() {
        let hash1 = calculate_sha1("Hello, World!");
        let hash2 = calculate_sha1("Hello, World!");
        // Ensure that identical inputs produce the same SHA1 hash
        assert_eq!(hash1, hash2);
    }
}
