pub fn xor_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut count = 0;
    while z != 0 {
        count += z & 1;
        z >>= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_distance() {
        assert_eq!(xor_distance(1, 4), 2);
        assert_eq!(xor_distance(0, 0), 0);
        assert_eq!(xor_distance(15, 0), 4);
        assert_eq!(xor_distance(1, 1), 0);
        assert_eq!(xor_distance(255, 0), 8);
        assert_eq!(xor_distance(255, 255), 0);
        assert_eq!(xor_distance(10, 20), 4);
    }
}
