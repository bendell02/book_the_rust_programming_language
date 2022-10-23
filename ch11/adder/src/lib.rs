pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test] 
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_v2() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was '{}'", result);
    }
}


// should panic
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }

    pub fn new2(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }
        else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests_shoud_panic {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than_100_v2() {
        Guess::new2(200);
    }
}



// Result<T, E>
#[cfg(test)]
mod tests_result {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

