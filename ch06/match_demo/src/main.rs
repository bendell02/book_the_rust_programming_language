
// ===================================================

// // 01. basic use of match
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     value_in_cents(Coin::Penny);
// }

// ===================================================

// // 02. enum with value

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quater from {:?}", state);
//             25
//         },
//     }
// }

// fn main() {
//     value_in_cents(Coin::Penny);
//     value_in_cents(Coin::Quarter(UsState::Alabama));
// }

// ===================================================

// // 03. match Option

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None  => None,
//         Some(i) => Some(i+1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("six : {:?}, none: {:?}", six, none);
// }

// ===================================================

// // 04. match _

// fn main() {
//     let some_u8 = 5u8;

//     match some_u8 {
//         1 => println!("one"),
//         3 => println!("three"),
//         5 => println!("five"),
//         7 => println!("seven"),
//         _ => (),
//     }
// }

// ===================================================

// 05. if let

fn main() {
    let some_u8 = Some(0u8);

    match some_u8 {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8 {
        println!("three");
    }
}

// ===================================================

