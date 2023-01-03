pub mod arch;
pub mod logger;
pub mod wasmer_integration;
pub mod overlay_core;
pub mod overlay_file_system;
pub mod overlay_key_value_store;
pub mod overlay_messaging;
pub mod overlay_ping;

#[no_mangle]
pub extern "C" fn add(first: i64, second: i64) -> i64 {
    first + second
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
