// fn find_largest_value(number_list: &[i32]) -> &i32{
//     let mut largest = &number_list[0];
    
//     for number in number_list{
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// struct Point<T>{
//     x: T,
//     y: T
// }

// impl<T> Point<T>{
//     fn x(&self) -> &T{
//         &self.x
//     }
// }


// impl Point<i32>{
//     fn say_something_integer(&self) -> i32{
//         let x = 256;
//         println!("int Point = {}", x);
//         x
//     }
// }

// impl Point<char>{
//     fn say_something(&self) -> &str{
//         let x = "Hello my cousin";
//         println!("char Point = {}", x);
//         x
//     }
// }

pub trait Summary {
    fn summarize(&self) -> String{
        String::from("Read more...")
    }
}

struct NewsArticle {
    author: String,
    title: String,
    text: String
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}:\n{}\n{}", self.author, self.title, self.text)
    }
}


struct Tweet {
    author: String,
    message: String
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.message)
    }
}


struct Empty{}
impl Summary for Empty{}

struct Point<X1, Y1>{
    x: X1,
    y: Y1
}



impl<X1, Y1> Point<X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point { x: self.x, y: other.y }
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.summarize());
}

pub fn returns_summarizable(is_article: bool) -> impl Summary {
    Tweet{
        author: String::from("Kate"),
        message: String::from("Some Kate's message")
    }
}


pub fn test_generics(){
    println!("\nGENERICS");
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);

    let point1 = Point{x: 1, y: 3};
    let point2 = Point{x: 'h', y: 'c'};
    // point1.say_something_integer();
    // point2.say_something();
    
    let point3 = Point {x: 3, y: 'h'};
    let point4 = Point {x: 1.5, y: "seven"};

    let point5 = point3.mixup(point4);

    println!("Mixed Point is x = {} and y = {}", point5.x, point5.y);

    let article = NewsArticle {
        author: String::from("Mark"),
        title: String::from("FRESH NEWS"),
        text: String::from("Something Good Happened")
    };
    notify(&article);

    let tweet = Tweet {author: String::from("Jeremy"), message: String::from("Nah You are lying")};
    notify(&tweet);

    println!("{}", Empty{}.summarize());
}

fn largest<T: std::cmp::PartialOrd>(items_list: &[T]) -> &T {
    let mut largest = &items_list[0];
    
    for item in items_list{
        if largest < item{
            largest = item;
        }
    }
    largest
}