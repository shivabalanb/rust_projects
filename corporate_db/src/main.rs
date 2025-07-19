use std::{collections::HashMap, io};
// add USER -> DEPARTMENT
// DEPARTMENT -> USER[]
// output DEPARTMENT -> sorted USERs
// sorted USERs -> DEPARTMENT
fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome to the corporate db!");

    println!("Which user would you like to add?");
    let mut user = String::new();

    io::stdin().read_line(&mut user).expect("failed");

    let user: String = match user.trim().parse() {
        Ok(user_str) => user_str,
        Err(_) => return,
    };

    println!("What department would you like to add {user} to?");
    let mut department = String::new();

    io::stdin().read_line(&mut department).expect("failed");

    let department: String = match department.trim().parse() {
        Ok(department_str) => department_str,
        Err(_) => return,
    };

    let users = db.entry(department).or_insert(Vec::new());

    if !users.contains(&user) {
        users.push(user);
    }

    println!("{:?}", db);
}
