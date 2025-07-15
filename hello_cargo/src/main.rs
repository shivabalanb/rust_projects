struct User {
    name: String,
    has_access: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let mut u1: User = User {
        name: String::from("shiv"),
        has_access: false,
    };
    let u2: User = User {
        name: u1.name,
        has_access: u1.has_access,
    };

    u1.name = String::from("hey");

    let c1 = Color(0,0,0);

    println!("hey {}", u1.name);
    println!("hey {}", u2.name);
}
