use sha1::{Sha1, Digest};

pub fn gen_sha1(data: &str) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(data);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();

    // convert the result to a hexadecimal string
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_sha1() {
        let data = "example data";
        let digest = gen_sha1(data);
        assert_eq!(digest, "9fc42adac31303d68b444e6129f13f6093a0e045");
    }
}
