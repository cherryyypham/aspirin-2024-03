/*!
    calculator.rs

    This file contains the implementation of the calculator program with basic logic operations and different bases for these operations.
*/
use std::io;

#[derive(Debug, Clone, Copy)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
}

impl BitwiseOperator {
    /// Converts a string to a `BitwiseOperator`.
    ///
    /// # Parameters
    /// `s`: A string slice that represents the operator. Can be either the symbol or its English name.
    ///
    /// # Returns
    /// An `Option` that contains the corresponding `BitwiseOperator` if the string matches one of the supported operators, otherwise `None`.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "&" | "and" => Some(BitwiseOperator::And),
            "|" | "or" => Some(BitwiseOperator::Or),
            "^" | "xor" => Some(BitwiseOperator::Xor),
            _ => None,
        }
    }

    /// Applies the bitwise operation to two numbers.
    ///
    /// # Parameters
    /// * `self`: The bitwise operator to apply.
    /// * `a`: The first number.
    /// * `b`: The second number.
    ///
    /// # Returns
    /// The result of applying the bitwise operation to `a` and `b`.
    pub fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            BitwiseOperator::And => a & b,
            BitwiseOperator::Or => a | b,
            BitwiseOperator::Xor => a ^ b,
        }
    }
}

/// Parses a number from a string with support for different bases (decimal, binary, hexadecimal).
///
/// # Parameters
/// `s`: A string slice that represents the number. May include prefixes `0x` for hexadecimal or `0b` for binary.
///
/// # Returns
/// A `Result` containing the parsed number as `u64` if successful, or an error message if parsing fails.
fn parse_number(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let (base, value) = if let Some(stripped) = s.strip_prefix("0x") {
        (16, stripped)
    } else if let Some(stripped) = s.strip_prefix("0b") {
        (2, stripped)
    } else {
        (10, s)
    };

    u64::from_str_radix(value, base).map_err(|e| e.to_string())
}

/// Performs a bitwise operation on two numbers based on the given operator.
///
/// # Parameters
/// * `first`: The first number as a string. Can be in decimal, binary, or hexadecimal format.
/// * `second`: The second number as a string. Can be in decimal, binary, or hexadecimal format.
/// * `operation`: The bitwise operation to perform as a string. Can be the symbol or the English name.
///
/// # Returns
/// A `Result` containing the result of the bitwise operation if successful, or an error message if any issue occurs.
fn calculate(first: &str, second: &str, operation: &str) -> Result<u64, String> {
    let num1 = parse_number(first)?;
    let num2 = parse_number(second)?;
    let op = BitwiseOperator::from_str(operation).ok_or_else(|| "Invalid operator".to_string())?;

    Ok(op.apply(num1, num2))
}

/// The main function that runs the calculator.
///
/// Prompts the user to enter two numbers and a bitwise operation.
/// Performs the calculation and prints the result.
fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).unwrap();
    let first = input.trim().to_string();
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).unwrap();
    let second = input.trim().to_string();
    input.clear();

    println!("Enter the desired operation:");
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim().to_string();

    match calculate(&first, &second, &operation) {
        Ok(result) => println!(
            "The result of {} {} {} is {}",
            first, operation, second, result
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_operations() {
        assert_eq!(calculate("0101", "0011", "XOR").unwrap(), 0110);
        assert_eq!(calculate("0101", "0011", "and").unwrap(), 0001);
        assert_eq!(calculate("0101", "0011", "|").unwrap(), 0111);
    }

    #[test]
    fn test_invalid_input() {
        assert!(calculate("12", "32", "skfks").is_err());
        assert!(calculate("sfk", "32", "XOR").is_err());
        assert!(calculate("12", "32", "XOR").is_ok());
    }
}
