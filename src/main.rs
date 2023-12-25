use vectors::tests_package::tests_module::{
    adder, 
    greeting,
    Guess,
    Rectangle,
    prints_and_returns_10
};


fn main() {
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    #[should_panic]
    fn another(){
        panic!("Make this test fail");
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, adder(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    
    #[test]
    fn larger_or_not(){
        let first_rectangle = Rectangle{width: 3, height: 10};
        let second_rectangle = Rectangle{width: 2, height: 3};
        assert!(first_rectangle.can_hold(&second_rectangle));
    }

    #[test]
    fn smaller_or_not(){
        let first_rectangle = Rectangle{width: 3, height: 10};
        let second_rectangle = Rectangle{width: 2, height: 15};
        assert!(!first_rectangle.can_hold(&second_rectangle));
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 0);
    }
}
