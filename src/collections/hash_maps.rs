use std::collections::HashMap;

pub fn test_maps(){
    println!("\nHASHMAPS");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores{
        println!("{key}: {value}");
    }
}