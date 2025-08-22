# Corporate Database

An interactive command-line application for managing a corporate database with users and departments.

## How to Use

1. **Build and run the project:**

   ```bash
   cargo run
   ```

2. **Follow the interactive menu:**
   - Press `1` to add a user to a department
   - Press `2` to view users in a department
   - Press `-1` to exit

## How it Works

This is an interactive database management system that:

- Stores users organized by departments using a HashMap
- Maintains sorted lists of users within each department
- Prevents duplicate users in the same department
- Provides a simple command-line interface for database operations

## Features

- **Add Users**: Add users to departments with automatic sorting
- **View Departments**: Display all users in a specific department
- **Duplicate Prevention**: Automatically prevents adding the same user twice to a department
- **Sorted Storage**: Users are automatically sorted alphabetically within each department
- **Interactive Interface**: Simple menu-driven command-line interface

## Example Usage

```
Welcome to the corporate db!

Press 1 to add user to a department
Press 2 to view db
Press -1 to exit

> 1
Which user would you like to add?
Alice
What department would you like to add User("Alice") to?
Engineering

> 2
What department would you like to view?
Engineering
Department Department("Engineering") has the following users: [User("Alice")]
```

## Project Structure

- `src/main.rs` - Main application logic and user interface
- Uses custom `User` and `Department` structs for type safety
- HashMap-based storage for efficient department-user relationships
