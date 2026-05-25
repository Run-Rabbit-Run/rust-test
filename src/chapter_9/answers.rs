use std::fs::File;
use std::io::ErrorKind;

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
pub fn check_7() {}

// TASK 8
pub fn check_8() {}

// TASK 9
pub fn check_9() {}

// TASK 10
pub fn check_10() {}
