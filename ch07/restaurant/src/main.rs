
// // 00. use std package
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);

//     println!("{:#?}", map);
// }


// 01. use external package
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{:#?}", secret_number);
}