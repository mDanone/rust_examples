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

fn test_parent(){}

mod test_parent_mod{
    fn test_child(){
        super::test_parent();
    }
}

pub fn test_enums(){

    let m = Message::Write(String::from("hello"));
    m.call();


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

