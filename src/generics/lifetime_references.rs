fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    }
    else{
        y
    }
}

fn main(){
    let string1 = String::from("abcd");
    let string2 = "xyz";
    {
        let result = longest_with_an_announcement(&string1, &string2, "THIS IS THE LONGEST STRING");
        println!("{}", result);
    }
    
    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt{
        part: first_sentence
    };
}