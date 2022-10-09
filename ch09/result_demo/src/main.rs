
// // 0. version 1
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");
    
//     let f = match f {
//         Ok(file) => file,
//         Err(error) =>  match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("There was a problem openning the file: {:?}", e),
//             },
//             other_error => panic!("There was a problem openning the file: {:?}", other_error),
//         },
//     };
// }

// // 1. version 2
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt").map_err(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("There was a problem openning the file: {:?}", error);
//             })
//         }
//         else {
//             panic!("There was a problem openning the file: {:?}", error);
//         }
//     });
// }

// // 2. version 3
// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").unwrap();
// }

// 3. version 4
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

