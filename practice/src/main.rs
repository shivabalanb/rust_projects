#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {width: size, height: size}
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn max(&self, other: Rectangle)->Rectangle{
        Rectangle{
            width:self.width.max(other.width),
            height: self.height.max(other.height)
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let mut rect1 = &Box::new(Rectangle {
        width: 30,
        height: 50,
    });
    
    let area = Rectangle::area(&&&&&&rect1);

    println!(
        "{}",area
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn first_words(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn generate_nth_fibonacci(n: u8) -> u32 {
    if n < 2 {
        n as u32
    } else {
        generate_nth_fibonacci(n - 2) + generate_nth_fibonacci(n - 1)
    }
}
