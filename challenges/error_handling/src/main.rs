use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;

fn main() {
    println!("hello world");
}

// EASY CHALLENGE 3: Basic error propagation with ?
// Goal: Learn to use ? operator effectively
mod easy_challenge_3 {
    use super::*;

    // TODO: Read file and count lines
    fn count_lines_in_file(filename: &str) -> Result<usize, io::Error> {
        // Read file and count number of lines
        // Use ? operator for error propagation
        let contents = fs::read_to_string(filename)?;
        let line_count = contents.lines().count();
        Ok(line_count)
    }

    // TODO: Parse config from file
    fn parse_config_number(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // Read file, parse first line as integer
        // Handle both IO errors and parse errors
        let contents = fs::read_to_string(filename)?;
        let first_line = contents.lines().next().ok_or("Empty file")?;
        let number = first_line.parse::<i32>()?;
        Ok(number)
    }

    // TODO: Chain multiple fallible operations
    fn process_numbers_file(filename: &str) -> Result<f64, Box<dyn std::error::Error>> {
        // Read file, parse each line as number, return average
        // Use ? operator throughout
        let contents = fs::read_to_string(filename)?;
        let mut sum = 0.0;
        let mut count = 0;
        for line in contents.lines() {
            let parsed: f64 = line.parse()?;
            sum += parsed;
            count += 1;
        }
        Ok(sum / count as f64)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_count_lines() {
            // Create test file
            fs::write("test_lines.txt", "line1\nline2\nline3").unwrap();

            let count = count_lines_in_file("test_lines.txt").unwrap();
            assert_eq!(count, 3);

            // Clean up
            fs::remove_file("test_lines.txt").ok();

            // Test non-existent file
            assert!(count_lines_in_file("nonexistent.txt").is_err());
        }
    }
}

mod easy_challenge_2 {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,
    }

    impl Person {
        fn new(name: String, age: u32, email: Option<String>) -> Self {
            Person { name, age, email }
        }

        // TODO: Get email domain if email exists
        fn email_domain(&self) -> Option<String> {
            // Extract domain from email (part after @)
            // Return None if no email or invalid format
            match &self.email {
                None => None,
                Some(email) =>  {
                    if let Some(domain) = email.split("@").last() {
                        Some(domain.to_string())
                    } else {
                        None
                    }
                },
            }
        }

        fn email_domain_idiomatic(&self) -> Option<String> {
            self.email.as_ref()?
                .split("@")
                .nth(1)
                .map(|s| s.to_string())
        }

        // TODO: Check if person is adult with email
        fn is_adult_with_email(&self) -> bool {
            // Return true only if age >= 18 AND has email
            if self.age >= 18 && self.email.is_some() {
                true
            } else {
                false
            }
        }

        fn is_adult_with_email_improved(&self) -> bool {
            self.age >= 18 && self.email.is_some()
        }

        // TODO: Get display name or default
        fn display_name(&self) -> String {
            // Return name, but if name is empty, return "Anonymous"
            if self.name.is_empty() {
                "Anonymous".to_string()
            } else {
                self.name.clone()
            }
        }
    }

    // TODO: Find person by email in a list
    fn find_person_by_email<'a>(people: &'a[Person], email: &str) -> Option<&'a Person> {
        // Find person with matching email
        for p in people {
            if let Some(p_email) = p.email.clone() {
                if p_email == email {
                    return Some(p);
                }
            }
        }
        None
    }

    fn find_person_by_email_improved<'a>(people: &'a [Person], email: &str) -> Option<&'a Person> {
        people.iter().find(|person| {
            person.email.as_ref() == Some(&email.to_string())
        })
    }

    fn find_person_by_email_elegant<'a>(people: &'a [Person], email: &str) -> Option<&'a Person> {
        people.iter().find(|person| {
            person.email.as_deref() == Some(email)
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_email_domain() {
            let person = Person::new("Alice".to_string(), 25, Some("alice@example.com".to_string()));
            assert_eq!(person.email_domain(), Some("example.com".to_string()));

            let no_email = Person::new("Bob".to_string(), 30, None);
            assert_eq!(no_email.email_domain(), None);
        }

        #[test]
        fn test_adult_with_email() {
            let adult_with_email = Person::new("Alice".to_string(), 25, Some("alice@example.com".to_string()));
            assert!(adult_with_email.is_adult_with_email());

            let minor_with_email = Person::new("Charlie".to_string(), 16, Some("charlie@example.com".to_string()));
            assert!(!minor_with_email.is_adult_with_email());

            let adult_no_email = Person::new("David".to_string(), 30, None);
            assert!(!adult_no_email.is_adult_with_email());
        }
    }
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
