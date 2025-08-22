# MiniGrep

A command-line text search tool built in Rust, similar to the Unix `grep` command.

## How to Use

1. **Build the project:**

   ```bash
   cargo build
   ```

2. **Run the search tool:**

   ```bash
   cargo run <query> <file_path>
   ```

3. **For case-insensitive search:**
   ```bash
   IGNORE_CASE=1 cargo run <query> <file_path>
   ```

## How it Works

This is a text search utility that:

- Takes a search query and file path as command-line arguments
- Searches through the specified file for lines containing the query
- Supports both case-sensitive and case-insensitive search modes
- Prints matching lines to the console
- Handles errors gracefully with informative error messages

## Features

- **Text Search**: Find lines containing specific text patterns
- **Case Sensitivity**: Choose between case-sensitive and case-insensitive search
- **Error Handling**: Robust error handling for file operations and argument parsing
- **Environment Variables**: Use `IGNORE_CASE` environment variable for case-insensitive search
- **Command-Line Interface**: Simple and intuitive command-line usage

## Example Usage

```bash
# Search for "nobody" in poem.txt (case-sensitive)
cargo run nobody poem.txt

# Search for "NOBODY" in poem.txt (case-insensitive)
IGNORE_CASE=1 cargo run NOBODY poem.txt

# Search for "rust" in any text file
cargo run rust src/main.rs
```

## Example Output

```
Searching for nobody
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
```

## Project Structure

- `src/main.rs` - Command-line interface and argument parsing
- `src/lib.rs` - Core search functionality and configuration
- `poem.txt` - Sample text file for testing
- `output.txt` - Example output file

## Dependencies

- Standard library only - no external dependencies required
- Uses `std::env` for environment variable handling
- Uses `std::fs` for file operations
