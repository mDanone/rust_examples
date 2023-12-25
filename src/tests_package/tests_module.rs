pub fn adder(value: i32) -> i32{
    value + 2
}


pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    pub value: i32,
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Guess value must be greeter than or equal to 1, got {}.", value);
        } else if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess {value}
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("The input value is {a}");
    10
}