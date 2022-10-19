
// // 00. lifetime parameter
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest stringis {}", result);
// // }


// // 01. lifetime parameter
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//     }

//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest stringis {}", result);
// }


// 02. struct lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a 'a'");

    let i = ImportantExcerpt{part: first_sentence};

}

