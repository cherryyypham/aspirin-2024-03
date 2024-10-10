/*!
    fizz_buzz.rs

    This file contains the implementation of the FizzBuzz problem.
    The goal is to print numbers from 1 to a given `max_num`, substituting multiples of 3 with "Fizz",
    multiples of 5 with "Buzz", and multiples of both with "FizzBuzz".
*/

/// Prints numbers from 1 to `max_num`, with certain exceptions:
/// * For multiples of 3, it prints "Fizz".
/// * For multiples of 5, it prints "Buzz".
/// * For multiples of both 3 and 5, it prints "FizzBuzz".
///
/// # Arguments
///
/// `max_num` - A positive integer (`u32`) that specifies the upper limit of the range to print.

/// Handle FizzBuzz rules for printing
pub fn print_fizz_buzz(max_num: u32) {
    for i in 1..=max_num {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

/// The main function that prints the FizzBuzz sequence
fn main() {
    // Change input as needed
    print_fizz_buzz(15);
}
