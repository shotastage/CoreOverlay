use sha1::{Sha1, Digest};

// Function to calculate SHA1 hash from a string
pub fn calculate_sha1(text: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

// Function to calculate SHA1 hash from a file
pub fn calculate_file_sha1(path: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
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
