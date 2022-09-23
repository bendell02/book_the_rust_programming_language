

// 01. struct
fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email:String, username: String) ->User {
        User{
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(String::from("b@example.com"), String::from("b"));

    let user3 = User{
        email: String::from("c@example.com"),
        username: String::from("c"),
        ..user1
    };

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:#?}", user1);
    println!("{:#?}", user2);
    println!("{:#?}", user3);

    println!("{:#?}", black);
    println!("{:#?}", origin);
}
