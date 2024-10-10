/*!
    guessing_game.rs

    Continuously take user inputs and give hints until the user correctly guesses the automatically generated number.
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Reads input from the user and returns it as an integer.
///
/// # Returns
///
/// * `i32` - The integer value entered by the user.
///
/// # Panics
///
/// This function will panic if the input cannot be parsed as an integer.
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// The main function that runs the number guessing game.
///
/// Generates a random number between 1 and 100 and continuously prompts the user to guess the number.
/// Provides hints if the guess is too high or too low and ends when the correct number is guessed.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_input();
        print!("You guessed: {}. ", guess);

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
