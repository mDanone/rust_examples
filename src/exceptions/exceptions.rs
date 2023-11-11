use std::{fs::File, io::{ErrorKind, self, Read}};

pub fn exception_handler(){
    // panic!();
    
    // let panic_vec = vec![1, 2, 3];
    // panic_vec[10];
    

    let greeting_file_result = File::open("hello.txt");

    // Вручную обрабатываем ошибки которые может вернуть
    // объект Result метода File::open
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    
    // unwrap или сам отдаст файл или вызовет panic!
    let file_with_wrap = File::open("hello2.txt").unwrap();

    // expect метод позволяет добавлять кастомное сообщение об ошибке
    let file_with_expect = File::open("expected").expect("Some Error message");

    let username_result = read_username_from_file();
    let username = username_result.expect("Can not read some file!");
    
}

// Функция которая может вернуть ошибку
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn part_of_number_resolving(){
    loop {
        let guess = "    3   ";
        let guess: i32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(err) => continue,
        };
        
        if (guess < 1) || (guess > 100){
            panic!("Number must be in 1 < n < 100");
        }
    }
}