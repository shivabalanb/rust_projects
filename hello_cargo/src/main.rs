fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    // let num=3;

    // if num > 5 {
    //     println!("High");
    // } else {
    //     println!("Low");
    // }

    // another_function(5);
}

fn another_function(num: i32) {
    println!("Hello, world! {:?}", num);
}
