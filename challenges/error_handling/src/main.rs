use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;

fn main() {
    println!("hello world");
}
// =============================================================================
// EASY CHALLENGES (1-3): Basic Result<T, E> and Option<T>
// =============================================================================

// EASY CHALLENGE 1: Basic Option and Result handling
// Goal: Understand unwrap, expect, and pattern matching
mod easy_challenge_1 {
    use super::*;

    // TODO: Implement safe division that handles division by zero
    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        // Return Some(result) if valid, None if division by zero
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    // TODO: Parse a string to integer with meaningful error
    fn parse_positive_number(input: &str) -> Result<u32, String> {
        // Parse string to u32, but return error if negative or invalid
        // Use custom error messages like "Invalid number format" or "Number must be positive"
        let parsed = input.parse::<u32>();

        if let Ok(n) = parsed {
            if n > 0 {
                Ok(n)
            } else {
                Err(format!("Number must be positive: {}", input))
            }
        } else {
            Err(format!("Invalid number format: {}", input))
        }
    }

    // TODO: Chain multiple operations that can fail
    fn calculate_average(numbers: Vec<&str>) -> Result<f64, String> {
        // Parse all strings to numbers, then calculate average
        // Handle: empty vec, parse errors, etc.
        if numbers.is_empty() {
            return Err("No numbers given".to_string())
        }

        let mut sum = 0.0;

        for num_str in &numbers {
            match num_str.parse::<f64>() {
                Ok(num) => sum += num,
                Err(_) => return Err(format!("Invalid number format: {}", num_str))
            }
        }

        Ok(sum / numbers.len() as f64)

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_safe_divide() {
            assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
            assert_eq!(safe_divide(10.0, 0.0), None);
            assert_eq!(safe_divide(0.0, 10.0), Some(0.0));
        }

        #[test]
        fn test_parse_positive() {
            assert!(parse_positive_number("42").is_ok());
            assert_eq!(parse_positive_number("42").unwrap(), 42);
            assert!(parse_positive_number("-5").is_err());
            assert!(parse_positive_number("abc").is_err());
        }

        #[test]
        fn test_calculate_average() {
            assert_eq!(calculate_average(vec!["10", "20", "30"]).unwrap(), 20.0);
            assert!(calculate_average(vec!["10", "abc", "30"]).is_err());
            assert!(calculate_average(vec![]).is_err());
        }
    }
}
