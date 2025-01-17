use sha1::{Digest, Sha1};

// Function to calculate SHA1 hash from a string
pub fn calculate_sha1(text: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sha1() {
        let text = "test";
        let hash = calculate_sha1(text);
        assert_eq!(hash, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    }
}
