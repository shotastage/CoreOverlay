// replicrypt/src/main.rs
use replicrypt::sha1::calculate_sha1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Display usage and exit if there are not enough arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <STRING>", args[0]);
        return;
    }

    // Extract the string specified by the user
    let input = &args[1];
    // Call the library's calculate_sha1 function
    let hash = calculate_sha1(input);
    // Output the hash value
    println!("{}", hash);
}
