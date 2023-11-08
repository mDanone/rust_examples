use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

pub fn task_tests(){
    let mut integer_list = vec![59, 2, 100, 100, 42, 42, 42, 42, 14, 60, 10];
    integer_list_stats(&mut integer_list);

    let mut latin_string = String::from("Hello World from Oustin Oscar");
    pig_latin(&mut latin_string);

    interactive_employee_accounting();
}


fn integer_list_stats(integer_list: &mut Vec<i32>){
    let mut count_nums : HashMap<i32, i8>= HashMap::new();
    
    insert_sort(integer_list);
    for item in &mut *integer_list{
        let count = count_nums.entry(*item).or_insert(0);
        *count += 1;
    }

    let sum_of_elements: i32 = integer_list.iter().sum::<i32>();
    println!("Sum of elements {}", sum_of_elements);
    println!("Median number {}", &integer_list[&integer_list.len() / 2]);
    
    let mut max_count: i8 = 0;
    let mut most_common_number = 0;
    for (k, v) in &count_nums{
        if *v > max_count{
            most_common_number = *k;
            max_count = *v;
        }
    }
    println!("Most common number is {}", most_common_number);
}


fn pig_latin(latin_string: &mut String){
    let mut result_string = String::from("");
    let vowels = "AEIOUYaeiouy";

    for word in latin_string.split_whitespace(){
        let mut new_word = word.to_owned();
        if vowels.contains(&word[..1]){
            new_word.push_str("-hay");
            result_string.push_str(&new_word);
        }
        else{
            new_word.push('-');
            new_word.push(word.chars().nth(0).unwrap());
            new_word.push_str("ay");
            result_string.push_str(&new_word);
        }
        result_string.push(' ');
    }
    println!("{}", result_string);
}


fn insert_sort(integer_list: &mut Vec<i32>){
   for mut index in 0..integer_list.len(){
        while index > 0 {
            if integer_list[index] < integer_list[index - 1]{
                integer_list.swap(index, index -1 );
                index -= 1;
            }
            else{
                break;
            }
        }
    }
}


fn interactive_employee_accounting(){
    let mut employee_accounter: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let _ = stdout().flush();
        let mut input_string  = String::new();
        stdin().read_line(&mut input_string);

        if input_string.to_lowercase() == "get result\n"{
            for (k, v) in &employee_accounter{
                println!("{}: {}", k, v.join(" "));
            }
        }
        let parsed_words: Vec<String> = input_string.trim().split(" ").map(|t| t.to_string()).collect();

        if parsed_words.len() < 4 {
            break;
        }
        else if (parsed_words[0].to_lowercase() != "add") & (parsed_words[2].to_lowercase() != "to") {
            break;
        }
        else{
            employee_accounter
            .entry(parsed_words[3].clone())
            .or_insert(Vec::new()).push(parsed_words[1].clone());
        }
    }
}