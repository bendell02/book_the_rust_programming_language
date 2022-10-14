
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

// // 3. version 4
// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }


// // 4. return Result to caller
// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read_username_from_file() -> Result<String,  io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
//     let rlt = read_username_from_file();

//     println!("{:?}", rlt);
// }


// // 5. return Result to caller with ?
// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read_username_from_file() -> Result<String,  io::Error> {
//     let mut f = File::open("hello.txt")?;

//     let mut s = String::new();

//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn main() {
//     let rlt = read_username_from_file();

//     println!("{:?}", rlt);
// }


// // 6. return Result to caller with ?
// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn read_username_from_file() -> Result<String,  io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     let rlt = read_username_from_file();

//     println!("{:?}", rlt);
// }


// // 7. simple presentation
// use std::io;
// use std::fs;

// fn read_username_from_file() -> Result<String,  io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let rlt = read_username_from_file();

//     println!("{:?}", rlt);
// }


// 8.()
use std::error::Error;
use std::fs::File;

fn main() ->Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

