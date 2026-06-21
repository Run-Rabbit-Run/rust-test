fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }

    Some(max)
}

pub fn check_1() {
    let array: Vec<i32> = vec![1, 2, 5, 123, 43, 14];
    let result = largest(&array);

    match result {
        Some(item) => println!("largest i32 = {item}"),
        None => (),
    }

    let array: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let result = largest(&array);

    match result {
        Some(item) => println!("largest char = {item}"),
        None => (),
    }

    let array: Vec<f64> = vec![4.2, 6.5, 11.4, 3.6];
    let result = largest(&array);

    match result {
        Some(item) => println!("largest f64 = {item}"),
        None => (),
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn swap(self) -> Point<U, T> {
        Point {
            x: self.y,
            y: self.x,
        }
    }
}

pub fn check_2() {
    let point = Point::new(1, 22);
    println!("x: {}, y: {}", point.x(), point.y());
    let point = point.swap();
    println!("After swap => x: {}, y: {}", point.x(), point.y());

    let point = Point::new(1, 22.5);
    println!("x: {}, y: {}", point.x(), point.y());
    let point = point.swap();
    println!("After swap => x: {}, y: {}", point.x(), point.y());

    let point = Point::new("Hello", String::from(", George"));
    println!("x: {}, y: {}", point.x(), point.y());
    let point = point.swap();
    println!("After swap => x: {}, y: {}", point.x(), point.y());
}

pub fn check_3() {}

enum Slot<T> {
    Empty,
    Filled(T),
}

impl<T> Slot<T> {
    fn is_empty(&self) -> bool {
        matches!(self, Slot::Empty)
    }

    fn is_filled(&self) -> bool {
        matches!(self, Slot::Filled(_))
    }

    fn replace(&mut self, value: T) -> Option<T> {
        let old_slot = std::mem::replace(self, Slot::Filled(value));

        match old_slot {
            Slot::Empty => None,
            Slot::Filled(old) => Some(old),
        }
    }
}

pub fn check_4() {
    let mut slot = Slot::Filled(String::from("Hi"));
    println!("Is empty: {}", slot.is_empty());
    println!("Is filled: {}", slot.is_filled());
    let replaced = slot.replace(String::from("Mom"));
    match replaced {
        None => println!("None"),
        Some(res) => println!("Res: {res}"),
    }

    let mut slot = Slot::Filled(32);
    println!("Is empty: {}", slot.is_empty());
    println!("Is filled: {}", slot.is_filled());
    let replaced = slot.replace(88);
    match replaced {
        None => println!("None"),
        Some(res) => println!("Res: {res}"),
    }
}

pub fn check_5() {}

pub fn check_6() {}

pub fn check_7() {}

pub fn check_8() {}

pub fn check_9() {}

pub fn check_10() {}

pub fn check_11() {}

pub fn check_12() {}

pub fn check_13() {}
