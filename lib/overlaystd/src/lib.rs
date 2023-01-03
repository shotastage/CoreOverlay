pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn humming_distance(left: usize, right: usize) -> usize {
    add(left, right)
}

#[no_mangle]
pub extern "C" fn c_humming_distance(left: usize, right: usize) -> usize {
    humming_distance(left, right)
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
