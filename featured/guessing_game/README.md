# Guessing Game

A classic number guessing game built in Rust where you try to guess a randomly generated number.

## How to Use

1. **Build and run the project:**

   ```bash
   cargo run
   ```

2. **Play the game:**
   - The game generates a random number between 1 and 100
   - Enter your guess when prompted
   - The game will tell you if your guess is too high or too low
   - Keep guessing until you find the correct number!

## How it Works

This is a simple number guessing game that:

- Generates a random secret number between 1 and 100
- Takes user input for guesses
- Provides feedback on whether the guess is too high or too low
- Continues until the correct number is guessed
- Handles invalid input gracefully by asking for a new guess

## Features

- **Random Number Generation**: Uses the `rand` crate to generate unpredictable numbers
- **Input Validation**: Handles non-numeric input gracefully
- **Interactive Feedback**: Provides clear feedback on each guess
- **Loop Gameplay**: Continues until the correct number is found
- **User-Friendly**: Clear prompts and instructions

## Example Gameplay

```
Guess the number!
Please input your guess.
50
You guessed: 50
Too small!
Please input your guess.
75
You guessed: 75
Too big!
Please input your guess.
62
You guessed: 62
You win!
```

## Dependencies

- `rand = "0.8.5"` - For random number generation

## Project Structure

- `src/main.rs` - Main game logic and user interaction
- Uses `rand::thread_rng()` for secure random number generation
- Implements comparison logic with `Ordering` enum
