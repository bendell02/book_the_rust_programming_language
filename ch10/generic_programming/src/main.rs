
// // 00. generic_programming in struct
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point{x: 5, y: 4};
//     let float = Point{x: 1.0, y: 4.0};
//     println!("{:?}", integer);
//     println!("{:?}", float);
// }


// // 01.  generic_programming in struct
// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point{x: 5, y: 4};
//     let float = Point{x: 1.0, y: 4.0};
//     let integer_and_float = Point{x: 5, y: 4.0};
//     println!("{:?}", integer);
//     println!("{:?}", float);
//     println!("{:?}", integer_and_float);
// }


// // 02. generic_programming in function
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) +self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point{x: 5, y: 10};
//     println!("p.x = {}", p.x());

//     let f = Point{x: 8.0, y: 6.0};
//     println!("f's distance is {}", f.distance_from_origin());
// }


// 03. generic_programming mixup
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point {x: 4, y:10.4};
    let p2 = Point {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3: {:?}", p3);
}