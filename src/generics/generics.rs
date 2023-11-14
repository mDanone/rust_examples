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


struct Point<X1, Y1>{
    x: X1,
    y: Y1
}


impl<X1, Y1> Point<X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point { x: self.x, y: other.y }
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