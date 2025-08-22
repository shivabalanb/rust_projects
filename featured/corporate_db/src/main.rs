use std::{collections::HashMap, io};
// add USER -> DEPARTMENT
// DEPARTMENT -> USER[]
// output DEPARTMENT -> sorted USERs
// sorted USERs -> DEPARTMENT
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct User(String);
#[derive(Debug, Eq, Hash, PartialEq)]
struct Department(String);

fn main() {
    let mut db: HashMap<Department, Vec<User>> = HashMap::new();
    println!("Welcome to the corporate db!");

    loop {
        println!("\n\n");
        println!("Press 1 to add user to a department");
        println!("Press 2 to view db");
        println!("Press -1 to exit");
        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("failed");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            -1 => break,
            1 => add_user_to_department(&mut db),
            2 => view_users_in_department(&mut db),
            _ => continue,
        }
    }

    println!("{:?}", db);
}

fn add_user_to_department(db: &mut HashMap<Department, Vec<User>>) {
    println!("Which user would you like to add?");
    let mut user = String::new();

    io::stdin().read_line(&mut user).expect("failed");

    let user: User = User(user.trim().to_string());

    println!("What department would you like to add {:?} to?", user);
    let mut department = String::new();

    io::stdin().read_line(&mut department).expect("failed");

    let department: Department = Department(department.trim().to_string());
    let department_name = department.0.clone();

    let users = db.entry(department).or_insert(Vec::new());

    if users.contains(&user) {
        println!(
            "User '{:?}' already exists in department '{:?}'. Not adding.",
            user, department_name
        );
        return;
    }

    let insert_idx = match users.binary_search(&user) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };

    users.insert(insert_idx, user);
}

fn view_users_in_department(db: &mut HashMap<Department, Vec<User>>) {
    println!("What department would you like to view?");
    let mut department = String::new();

    io::stdin().read_line(&mut department).expect("failed");

    let department: Department = Department(department.trim().to_string());

    match db.get(&department) {
        Some(users) => println!(
            "Department {:?} has the following users: {:?}",
            &department, &users
        ),
        None => println!("No users in {:?}", &department),
    };
}
