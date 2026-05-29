use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// TASK 1
fn print_number_at(numbers: &[i32], index: usize) {
    match numbers.get(index) {
        Some(num) => println!("{num}"),
        None => println!("Value with index {index} doesn't exist"),
    }
}

pub fn check_1() {
    let array = vec![22, 33, 44];
    print_number_at(&array, 0);
    print_number_at(&array, 2);
    print_number_at(&array, 99);
}

// TASK 2
fn calculate_discount_price(price: u32, discount_percent: u32) -> u32 {
    if discount_percent > 100 {
        panic!("Discount can't be more than 100. Now it's {discount_percent}");
    }

    price - (price * discount_percent / 100)
}

pub fn check_2() {
    println!("Cost: {}", calculate_discount_price(999, 0));
    println!("Cost: {}", calculate_discount_price(999, 25));
    println!("Cost: {}", calculate_discount_price(999, 100));
    // Panic, потому что продолжать работу опасно с некорректными данными.
}

// TASK 3
fn parse_age(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.parse::<u8>()
}

pub fn check_3() {
    for input in ["25", "abc", "300"] {
        match parse_age(input) {
            Ok(age) => println!("{age}"),
            Err(e) => println!("{e}"),
        }
    }
}

// TASK 4
fn open_file_with_message(path: &str) {
    match File::open(path) {
        Ok(_) => println!("File open"),
        Err(e) => println!("{path} => {e}"),
    }
}

pub fn check_4() {
    open_file_with_message("Cargo.toml");
    open_file_with_message("missing.txt");
}

// TASK 5
fn open_or_create(path: &str) -> File {
    match File::open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                File::create(path).unwrap_or_else(|error| panic!("Can't create a file {error}"))
            }
            _ => panic!("Can't open file {path}: {e}"),
        },
    }
}

pub fn check_5() {
    open_or_create("chapter_9_note.txt");
}

// TASK 6
pub fn check_6() {
    File::open("Cargo.toml").unwrap();
    File::open("Cargo.toml").expect("Cargo.toml должен существовать в корне проекта");
}

// TASK 7
fn read_file_manual(path: &str) -> Result<String, io::Error> {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn check_7() {
    match read_file_manual("Cargo.toml") {
        Ok(string) => println!("{string}"),
        Err(e) => println!("Error, {e}"),
    }
    match read_file_manual("missing.txt") {
        Ok(string) => println!("{string}"),
        Err(e) => println!("Error, {e}"),
    }
}

// TASK 8
fn read_file_short(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_builtin(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub fn check_8() {
    for input in ["Cargo.toml", "missing.txt"] {
        match read_file_short(input) {
            Ok(string) => println!("File {input}:\n{string}"),
            Err(e) => println!("File {input} error:\n{e}"),
        }

        match read_file_builtin(input) {
            Ok(string) => println!("File {input}:\n{string}"),
            Err(e) => println!("File {input} error:\n{e}"),
        }
    }
}

// TASK 9
pub fn check_9() {}

// TASK 10
pub fn check_10() {}
