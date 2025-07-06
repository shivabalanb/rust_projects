use std::vec;

fn main() {

    let mut v = vec![1,2,3];
    let num = v[2];
    v.push(2);
    println!("{}",num);

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

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn another_function(num: i32) {
    println!("Hello, world! {:?}", num);
}
