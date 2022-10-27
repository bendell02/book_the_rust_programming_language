
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

// cargo run 
// cargo run needle haystack
// cargo run test sample.txt


