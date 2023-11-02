
enum IpAddrKind {
    V4(String),
    V6(String)
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self, ){
        println!("Hello",);
    }
}

enum SomeValues{
    Int(u32),
    Float(f32),
    Text(String)
}

pub fn test_enums(){
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('h');
    let absent_number: Option<i32> = None;

    let mut enum_vec = vec![
        SomeValues::Int(2),
        SomeValues::Float(10.0),
        SomeValues::Text(String::from("Hello world"))
    ];
    for enum_value in enum_vec.iter_mut(){
        match enum_value{
            SomeValues::Int(value) => println!("This is SomeValues integer {}", value),
            SomeValues::Float(value) => println!("This is SomeValues floating {}", value),
            SomeValues::Text(value) => println!("This is SomeValues text {}", value),
        }
    };



}

