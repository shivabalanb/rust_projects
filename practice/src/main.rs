#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let rect1 = Rectangle{width: 30, height:50};

    println!("The are of the rectangle is {} square pixels.", rect1.area());

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
