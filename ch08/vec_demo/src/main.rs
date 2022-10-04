
fn main() {
     let v = vec![1, 2, 3];
    println!("v: {:#?}", v);

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("v2: {:#?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);
    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let v4 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v4[100];   // wrong
    let does_not_exist = v4.get(100);

    // // mutable borrow
    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v5[0];
    // v5.push(6);
    // println!("The first element is : {}", first);

    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("element in v6: {}", i);
    }

    let mut v7 = vec![101, 33, 58];
    for i in &mut v7 {
        *i += 50;
    }
    for i in &v7 {
        println!("element in v7: {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
