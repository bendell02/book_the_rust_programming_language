
// =======================================

// // 01. version 01
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let home = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     println!("Hello, world!");
// }

// =======================================

// // 02. version 02
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));

//     let loopback = IpAddrKind::V6(String::from("::1"));

//     println!("Hello, world!");
// }

// =======================================

// // 03. version 03
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddrKind::V4(127, 0, 0, 1);

//     let loopback = IpAddrKind::V6(String::from("::1"));

//     println!("Hello, world!");
// }

// =======================================

// // 04. another enumeration
// enum Message {
//     Quit,
//     Move{x: i32, y:i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("call");
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));

//     m.call();

//     println!("Hello, world!");
// }

// =======================================

// 05.Option

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("Hello, world!");
}

// =======================================
