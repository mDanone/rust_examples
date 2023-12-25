pub fn test_strings_in_utf(){
    println!("\nSTRINGS!!!");
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
