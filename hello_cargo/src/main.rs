use std::vec;

fn main() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    // let mut v = vec![1,2,3];
    // let num = v[2];
    // v.push(2);
    // println!("{}",num);

    // let m1 = String::from("Hello");
    // let m2 = String::from("world");
    // let (m1, m2) = greet(m1, m2);
    // let s = format!("{} {}", m1, m2);

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter;
    //     }
    // };

    // let num=3;

    // if num > 5 {
    //     println!("High");
    // } else {
    //     println!("Low");
    // }

    // another_function(5);
}

fn add_big_strings_simplified(dst: &mut Vec<String>, src: &[String]) {
    // Line A: Immutable borrow of `dst` occurs here, via `largest`.
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

    // Imagine 's.clone()' was replaced by 'String::from("example")'
    // Line B: Attempt to acquire a *mutable borrow* of `dst` for `push`.
    dst.push(String::from("example")); // This line would still error

    // Even here, after the push, largest might still be considered active by the borrow checker
    // if its lifetime extends to the end of the function and there were no more uses.
    // However, the error occurs precisely at the `dst.push` line.
    // println!("{}",largest);
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn another_function(num: i32) {
    println!("Hello, world! {:?}", num);
}
