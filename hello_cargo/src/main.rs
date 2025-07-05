fn main() {
    // another_function(5);

    let num=3;

    if num > 5 {
        println!("High");
    } else {
        println!("Low");
    }
}

fn another_function(num: i32) {
    println!("Hello, world! {:?}", num);
}
