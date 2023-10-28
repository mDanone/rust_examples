enum SomeValues{
    Int(u32),
    Float(f32),
    Text(String)
}

fn main() {
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
