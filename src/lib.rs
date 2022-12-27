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
