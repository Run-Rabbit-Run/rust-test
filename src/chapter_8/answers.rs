use std::collections::HashMap;

// TASK 1
pub fn check_1() {
    let temperatures: Vec<i32> = vec![18, 20, 21, 19, 17, 22, 23];

    if temperatures.is_empty() {
        println!("Temperatures are empty");
        return;
    }

    for temperature in &temperatures {
        println!("Temperature: {temperature}");
    }

    let first_temperature = temperatures.first().unwrap();
    let sum: i32 = temperatures.iter().sum();
    let average_temperature = sum as f64 / temperatures.len() as f64;
    let max_temperature = temperatures.iter().max().unwrap();
    let min_temperature = temperatures.iter().min().unwrap();

    println!("First: {first_temperature}");
    println!("Average: {average_temperature}");
    println!("Max: {max_temperature}");
    println!("Min: {min_temperature}");
}

// TASK 2
fn print_numbered_list(elements: &[String]) {
    for (i, element) in elements.iter().enumerate() {
        println!("{}: {}", i + 1, element);
    }
}

pub fn check_2() {
    let mut goods: Vec<String> = Vec::new();
    goods.push(String::from("Bread"));
    goods.push(String::from("Milk"));
    goods.push(String::from("Apple"));
    goods.push(String::from("Eggs"));
    goods.push(String::from("Vodka"));

    print_numbered_list(&goods);

    let removed_item = goods.pop();
    match removed_item {
        Some(g) => println!("{g} was deleted"),
        None => println!("List was empty"),
    }

    print_numbered_list(&goods);
}

// TASK 3
fn analyze_numbers(numbers: &[i32]) {
    if numbers.is_empty() {
        println!("We don't have numbers");
        return;
    }

    let numbers_len = numbers.len();
    let sum: i32 = numbers.iter().sum();
    let average_value = sum as f64 / numbers_len as f64;
    let mut bigger_then_average_count: i32 = 0;

    for number in numbers {
        if *number as f64 > average_value {
            bigger_then_average_count += 1;
        }
    }

    println!("We have {} numbers", numbers_len);
    println!("Sum: {sum}");
    println!("Average value: {average_value}");
    println!("Bigger then average value: {bigger_then_average_count}");
}

pub fn check_3() {
    let mut numbers: Vec<i32> = vec![2, 36, 12, 7, 0, 12, 34, 18];

    analyze_numbers(&numbers);

    for number in &mut numbers {
        *number += 10;
    }

    analyze_numbers(&numbers);
}

// TASK 4
pub fn check_4() {
    let mut greeting = String::from("Привет");
    greeting.push(',');
    greeting.push_str(" Георгий");
    greeting.push('!');

    println!("Предложение: {greeting}");
    println!("Количество байт: {}", greeting.len());
    println!("Количество символов: {}", greeting.chars().count());

    // len показывает количество байтов, а мы видим символы. Русские буквы состоят из двух байтов
}

// TASK 5
pub fn check_5() {
    let text = String::from("Здравствуйте");
    println!("Количество байт: {}", text.len());

    for ch in text.chars() {
        println!("{ch}");
    }

    for byte in text.bytes() {
        println!("{byte}");
    }

    println!("Количество символов: {}", text.chars().count());
}

// TASK 6
fn normalize_spaces(text: &str) -> String {
    let words = text.split_whitespace();
    let mut result = String::new();
    for word in words {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}
pub fn check_6() {
    let text = "  Rust   любит   точность  ";
    let clean_text = normalize_spaces(text);
    println!("{clean_text}");
}

// TASK 7
pub fn check_7() {
    let mut command_score_map: HashMap<String, i32> = HashMap::new();
    command_score_map.insert(String::from("MU"), 8);
    command_score_map.insert(String::from("Leeds"), 0);
    command_score_map.insert(String::from("Zenit"), 1);

    for (key, value) in &command_score_map {
        println!("{key}: {value}");
    }

    let mu_score = command_score_map.get("MU").copied().unwrap_or(0);
    println!("MU score: {mu_score}");

    let forbidden_team = command_score_map.get("Chelsea").copied().unwrap_or(0);
    println!("Forbidden team: {forbidden_team}");
}

// TASK 8
fn print_user_email(users: &HashMap<String, String>, name: &str) {
    match users.get(name) {
        Some(mail) => println!("{mail}"),
        None => println!("Oops"),
    }
}
pub fn check_8() {
    let mut users: HashMap<String, String> = HashMap::new();

    users.insert(String::from("Alice"), String::from("alice@mail.com"));
    print_user_email(&users, "Alice");

    users.insert(String::from("Alice"), String::from("new_alice@mail.com"));
    print_user_email(&users, "Alice");

    users
        .entry(String::from("Bob"))
        .or_insert("bob@mail.com".to_string());
    print_user_email(&users, "Bob");

    users
        .entry(String::from("Bob"))
        .or_insert("new_bob@mail.com".to_string());
    print_user_email(&users, "Bob");

    for (user, mail) in &users {
        println!("{user}: {mail}");
    }
}

// TASK 9
fn count_words(text: &str) -> HashMap<String, i32> {
    let mut result = HashMap::new();

    for word in text.split_whitespace() {
        let count = result.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    result
}
pub fn check_9() {
    let text = String::from("Rust rust ownership borrowing Rust");
    let map = count_words(&text);

    for (key, value) in &map {
        println!("{key} -> {value}");
    }
}

// TASK 10
pub fn check_10() {

}
