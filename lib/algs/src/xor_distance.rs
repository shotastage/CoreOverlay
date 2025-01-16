
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
        let test_cases = vec![
            (1, 4, 2),
            (0, 0, 0),
            (15, 0, 4),
            (1, 1, 0),
            (255, 0, 8),
            (255, 255, 0),
            (10, 20, 4),
        ];

        for (x, y, expected) in test_cases {
            let result = xor_distance(x, y);
            println!("xor_distance({}, {}) = {}", x, y, result);
            assert_eq!(result, expected);
        }
    }
}
