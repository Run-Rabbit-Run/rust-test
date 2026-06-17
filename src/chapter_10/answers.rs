fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut max  = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }

    Some(max)
}

pub fn check_1() {
    let array: Vec<i32> = vec!(1, 2, 5, 123, 43, 14);
    let result = largest(&array);

    match result {
        Some(item) => println!("largest i32 = {item}"),
        None => (),
    }
    
    let array: Vec<char> = vec!('a', 'b', 'c', 'd');
    let result = largest(&array);

    match result {
        Some(item) => println!("largest char = {item}"),
        None => (),
    }

    let array: Vec<f64> = vec!(4.2, 6.5, 11.4, 3.6);
    let result = largest(&array);

    match result {
        Some(item) => println!("largest f64 = {item}"),
        None => (),
    }
}


pub fn check_2() {

}


pub fn check_3() {

}


pub fn check_4() {

}


pub fn check_5() {

}


pub fn check_6() {

}


pub fn check_7() {

}


pub fn check_8() {

}


pub fn check_9() {

}


pub fn check_10() {

}


pub fn check_11() {

}


pub fn check_12() {

}


pub fn check_13() {

}

