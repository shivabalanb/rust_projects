use std::ops::Deref;
use crate::List::{Cons, Nil};

enum List{
    Cons(i32, Box<List>),
    Nil
}

fn main(){
   let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self)->&Self::Target{
        &self.0
    }
}

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        println!("most_stocked()");
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more…)")
    }
}

impl Summary for i32 {}

mod test {

    #[derive(Debug)]
    pub struct Rectangle {
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
            Self {
                width: size,
                height: size,
            }
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn max(&self, other: Rectangle) -> Rectangle {
            Rectangle {
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            }
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
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
