
fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents 2".to_string();
    println!("{}", s);

    let s = String::from("initial contents 3");
    println!("{}", s);

    let hello = String::from("你好");
    println!("{}",  hello);

    let mut s = String::from("foo ");
    s.push_str("bar");
    println!("{}", s);

    // push_str does not take ownership
    let mut s1 = String::from("foo ");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved, and can not be used again. + = add()
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s : {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{} - {} - {}", s1, s2, s3);
    println!("s: {}", s);

    let s1 = String::from("hello");
    // let h = s1[0];  //wrong
}
