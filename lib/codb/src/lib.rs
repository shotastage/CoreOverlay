// Import necessary modules
use bytes::Bytes;
use ethereum_types::H256;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

struct Database; // Define the Database struct

struct OverlayDB {
    backing: Arc<Database>,
    overlay: RwLock<HashMap<H256, Option<Bytes>>>,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
