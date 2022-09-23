
// =====================================
// // 01. variable
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels. ", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width*height
// }
// =====================================

// // 02.tuple
// fn main() {
//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels. ", area(rect1));
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0*dimension.1
// }

// =====================================

// // 03. struct
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle{width: 30, height: 50};

//     println!("rect1 is {:?}", rect1);
//     println!("rect1 is {:#?}", rect1);

//     println!("The area of the rectangle is {} square pixels. ", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// =====================================

// // 04. function
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle{width: 30, height: 50};

//     println!("rect1 is {:?}", rect1);
//     println!("rect1 is {:#?}", rect1);

//     println!("The area of the rectangle is {} square pixels. ", rect1.area());
// }

// =====================================

// // 05. function with more parameters
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle{width: 30, height: 50};
//     let rect2 = Rectangle{width: 10, height: 40};
//     let rect3 = Rectangle{width: 60, height: 45};

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// =====================================

// 06. associated function
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle::square(30);

    println!("rect1 is {:?}", rect1);
}

// =====================================
