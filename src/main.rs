enum SomeValues{
    Int(u32),
    Float(f32),
    Text(String)
}

enum IpAddrKind{
    V4(String),
    V6(String)
}

struct QuitMessage;
struct  MoveMessage{
    x: i32, y: i32
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

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

fn test_enums(){
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let some_number = Some(5);
    let some_char = Some('h');
    let absent_number: Option<i32> = None;
}


#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn test_control_flow(){
    value_in_cents(Coin::Quarter(UsState::Alabama));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
}


fn test_vectors(){
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2 = vec![1, 2, 3];
    vec1.push(2);
    vec2.push(3);
    let first_vec1 = &vec1[0];
    println!("vec1 first values is {}", first_vec1);
    
    let second_vec2: Option<&u32> = vec2.get(1);
    match second_vec2 {
        Some(second_vec2) => println!("The vec2 second value is {second_vec2}"),
        None => println!("There is no second value for vec2")
    }
    
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

fn test_strings_in_utf(){
    let mut s1 = String::new();
    s1.push_str("foobar");

    let mut s2 = String::from("not ");
    s2.push('h');
    s2.push('e');
    s2.push('l');
    s2.push('l');
    
    let s3 = format!("{s1}-{s2}");
    println!("{s3}, {s1}, {s2}");

    let hello = "Здравствуйте!";
    for sym in hello.chars(){
        print!("{sym}");
    }
    println!();
    for sym in hello.bytes(){
        print!("{sym}");
    }
    println!();
}

fn main() {
    test_vectors();
    test_strings_in_utf();
    test_enums();
    test_control_flow();
}
