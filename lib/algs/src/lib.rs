pub fn humming_distance(x: i32, y: i32) -> i32 {
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
    fn test_humming_distance() {
        assert_eq!(humming_distance(1, 4), 2);
        assert_eq!(humming_distance(0, 0), 0);
        assert_eq!(humming_distance(15, 0), 4);
        assert_eq!(humming_distance(1, 1), 0);
        assert_eq!(humming_distance(255, 0), 8);
        assert_eq!(humming_distance(255, 255), 0);
        assert_eq!(humming_distance(10, 20), 4);
    }
}
