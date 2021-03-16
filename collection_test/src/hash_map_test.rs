use std::collections::HashMap;

pub fn new() {
    let mut three_primary_color = HashMap::new();
    three_primary_color.insert(1, ThreePrimaryColors::Red);
    three_primary_color.insert(2, ThreePrimaryColors::Blue);
    three_primary_color.insert(3, ThreePrimaryColors::Green);

    for (k, v) in &three_primary_color {
        println!("{}:{:?}", k, v);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (k, v) in scores {
        println!("{}:{}", k, v);
    }
}

#[derive(Debug)]
enum ThreePrimaryColors {
    Red,
    Blue,
    Green,
    None,
}

pub fn lets_panic() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}", field_name);
}

pub fn get_val(str: &str) {
    let mut map = HashMap::new();
    map.insert(String::from("blue"), ThreePrimaryColors::Blue);
    map.insert(String::from("red"), ThreePrimaryColors::Red);

    let color = map.get(str);
    println!("{:#?}", color);
    let color = match map.get(str) {
        Some(c) => {
            println!("{:#?}", c);
            c
        }
        None => {
            println!("there is nothing");
            &ThreePrimaryColors::None
        }
    };
    println!("{:#?}", color);
}

pub fn insert_test(str: &str) {
    let mut map = HashMap::new();
    map.insert(str, 1);
    map.insert(str, 20);

    if let Some(n) = map.get(str) {
        println!("{}", n);
    }
}

pub fn insert_test_1(str: &str) {
    let mut map = HashMap::new();
    map.insert(str, 10);

    map.entry("Yellow").or_insert(50);
    map.entry(str).or_insert(30);
    if let Some(n) = map.get(str) {
        println!("{}", n);
    }
}

pub fn insert_test_2() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
