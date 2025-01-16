pub fn humming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut count = 0;
    while z != 0 {
        count += z & 1;
        z >>= 1;
    }
    count
}
