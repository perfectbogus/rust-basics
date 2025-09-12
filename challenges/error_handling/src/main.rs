use std::fs;
use std::io;
use std::num::ParseIntError;
use std::fmt;

fn main() {
    println!("hello world");
}

// MEDIUM CHALLENGE 6: Error handling in iterators and combinators
// Goal: Handle errors in functional programming style
mod medium_challenge_6 {
    use super::*;

    #[derive(Debug)]
    enum DataProcessingError {
        InvalidFormat(String),
        OutOfRange(i32),
        ProcessingFailed(String),
    }

    impl fmt::Display for DataProcessingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: Implement Display
            match self {
                DataProcessingError::InvalidFormat(msg) => write!(f, "Invalid input format: {}", msg),
                DataProcessingError::OutOfRange(n) => write!(f, "Out of range: {}", n),
                DataProcessingError::ProcessingFailed(msg) => write!(f, "{}", msg),
            }
        }
    }

    // Next Implementation
    impl std::error::Error for DataProcessingError {}

    // TODO: Process list of strings, parse to numbers, filter and transform
    fn process_data_pipeline(input: Vec<&str>) -> Result<Vec<i32>, DataProcessingError> {
        // 1. Parse each string to i32
        // 2. Filter out numbers not in range 1-100
        // 3. Square each number
        // 4. Return error if any step fails
        // Use iterator methods like collect(), map(), filter()
        input.iter()
            .map(|s| {
                s.parse::<i32>().map_err(|_| DataProcessingError::InvalidFormat(s.to_string()))
            })
            .map(|result| {
                result.and_then(|n| {
                    if n >= 1 && n <= 100 {
                        Ok(n*n)
                    } else {
                        Err(DataProcessingError::OutOfRange(n))
                    }
                })
            })
            .collect()
    }

    // TODO: Batch process with partial success
    fn batch_process(inputs: Vec<&str>) -> (Vec<i32>, Vec<DataProcessingError>) {
        // Process each input, collect successes and errors separately
        // Don't stop on first error - process all inputs
        let mut successes = Vec::new();
        let mut errors = Vec::new();
        inputs.iter().for_each(|s| {
            match s.parse::<i32>() {
                Ok(n) => successes.push(n),
                Err(e) => errors.push(DataProcessingError::InvalidFormat(e.to_string()))
            }
        });
        (successes, errors)
    }

    fn batch_process_functional(inputs: Vec<&str>) -> (Vec<i32>, Vec<DataProcessingError>) {
        let results: Vec<_> = inputs.iter()
            .map(|s| s.parse::<i32>().map_err(|e| DataProcessingError::InvalidFormat(e.to_string())))
            .collect();

        let mut successes = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            match result {
                Ok(n) => { successes.push(n); }
                Err(e) => { errors.push(e); }
            }
        }

        (successes, errors)
    }

    // TODO: Find first valid result
    fn find_first_valid(inputs: Vec<&str>) -> Option<i32> {
        // Try to parse each input, return first successful parse
        // Return None if all fail
        inputs.iter()
            .map(|s| s.parse::<i32>())
            .find(|result| result.is_ok())
            .map(|result| result.unwrap())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_data_pipeline() {
            let input = vec!["10", "20", "30"];
            let result = process_data_pipeline(input).unwrap();
            assert_eq!(result, vec![100, 400, 900]); // 10², 20², 30²

            let invalid_input = vec!["10", "abc", "30"];
            assert!(process_data_pipeline(invalid_input).is_err());
        }
        #[test]
        fn test_playground() {
            let input = vec!["10", "20", "30", "10"];
            let map_one = &input.iter()
                .map(
                    |s| s.parse::<i32>().map_err(|_| DataProcessingError::InvalidFormat(s.to_string()))
                ).collect::<Result<Vec<i32>, _>>();

            println!("{:?}", map_one);

            let map_two = &input.iter()
                .map(|s| {
                    s.parse::<i32>()
                        .map_err(|_| DataProcessingError::InvalidFormat(s.to_string()))
                })
                .map(|result| {
                    result.and_then(|n| {
                        if n >= 1 && n <= 100 {
                            Ok(n*n)
                        } else {
                            Err(DataProcessingError::OutOfRange(n))
                        }
                    })
                })
                .collect::<Result<Vec<i32>, _>>();

            println!("{:?}", map_two);
        }

        #[test]
        fn test_invalid_format_input() {
            let input = vec!["10", "20", "30", "ErrorFormat"];

            let result = process_data_pipeline(input);

            assert!(result.is_err());

            match result.unwrap_err() {
                DataProcessingError::InvalidFormat(e) => assert!(e.contains("Format")),
                others => panic!("Invalid format expected, got {}", others)
            }
        }

        #[test]
        fn test_batch_process() {
            let input = vec!["10", "abc", "20", "xyz", "30"];
            let (successes, errors) = batch_process(input);
            assert_eq!(successes.len(), 3); // 10, 20, 30
            assert_eq!(errors.len(), 2);    // abc, xyz
        }

        #[test]
        fn test_find_first_valid() {
            let input = vec!["10", "20", "30", "ErrorFormat"];
            let result = find_first_valid(input);

            assert_eq!(result, Some(10));
        }
    }
}


// MEDIUM CHALLENGE 5: Error conversion and From trait
// Goal: Automatic error conversion between different error types
mod medium_challenge_5 {
    use super::*;

    #[derive(Debug)]
    enum FileProcessingError {
        IoError(io::Error),
        ParseError(ParseIntError),
        ValidationError(String),
        EmptyFile,
    }

    impl fmt::Display for FileProcessingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: Implement Display
            match self {
                FileProcessingError::IoError(e) => write!(f, "I/O error: {}", e),
                FileProcessingError::ParseError(e) => write!(f, "Parse error: {}", e),
                FileProcessingError::ValidationError(e) => write!(f, "Validation error: {}", e),
                FileProcessingError::EmptyFile => write!(f, "Empty file"),
            }
        }
    }

    impl std::error::Error for FileProcessingError {}

    // TODO: Implement From traits for automatic conversion
    impl From<io::Error> for FileProcessingError {
        fn from(error: io::Error) -> Self {
            // TODO: Convert io::Error to FileProcessingError
            FileProcessingError::IoError(error)
        }
    }

    impl From<ParseIntError> for FileProcessingError {
        fn from(error: ParseIntError) -> Self {
            // TODO: Convert ParseIntError to FileProcessingError
            FileProcessingError::ParseError(error)
        }
    }

    fn process_number_file(filename: &str) -> Result<Vec<i32>, FileProcessingError> {
        // TODO: Read file, parse each line as i32, validate all numbers are positive
        // Use ? operator - errors should automatically convert
        let contents = fs::read_to_string(filename)?;
        let mut numbers = Vec::new();

        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let num = line.parse::<i32>()?;

            if num <= 0 {
                return Err(FileProcessingError::ValidationError(
                    "All numbers must be positive".to_string())
                );
            }
            numbers.push(num);
        }

        if numbers.is_empty() {
            return Err(FileProcessingError::EmptyFile)
        }

        Ok(numbers)
    }

    fn find_max_in_file(filename: &str) -> Result<i32, FileProcessingError> {
        // TODO: Use process_number_file and find maximum
        // Return ValidationError if no numbers found
        let numbers = process_number_file(filename)?;

        numbers.iter()
            .max()
            .copied()
            .ok_or(FileProcessingError::ValidationError("No maximum found".to_string()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_error_conversion() {
            // Test will create files and test error conversion
            fs::write("test_numbers.txt", "10\n20\n30").unwrap();

            let numbers = process_number_file("test_numbers.txt").unwrap();
            assert_eq!(numbers, vec![10, 20, 30]);

            let max = find_max_in_file("test_numbers.txt").unwrap();
            assert_eq!(max, 30);

            // Clean up
            fs::remove_file("test_numbers.txt").ok();
        }

        #[test]
        fn test_io_error() {
            let result = process_number_file("nonexistent_file.txt");

            assert!(result.is_err());
            match result.unwrap_err() {
                FileProcessingError::IoError(_) => {},
                other => panic!("Expected IoError, get: {:?}", other)
            }
        }

        #[test]
        fn test_validation_error() {
            fs::write("test_validation.txt", "10\n-5\n30").unwrap();

            let result = process_number_file("test_validation.txt");
            assert!(result.is_err());
            match result.unwrap_err() {
                FileProcessingError::ValidationError(msg) => {
                    assert!(msg.contains("positive"));
                },
                other => panic!("Expected ValidationError, got: {:?}", other)
            }

            fs::remove_file("test_validation.txt").ok();
        }

        #[test]
        fn test_empty_file_error() {
            fs::write("test_empty.txt", "").unwrap();

            let result = process_number_file("test_empty.txt");
            assert!(result.is_err());

            match result.unwrap_err() {
                FileProcessingError::EmptyFile => {},
                other => panic!("Expected EmptyFileError, got : {:?}", other)
            }

            fs::remove_file("test_empty.txt").ok();
        }

        #[test]
        fn test_whitespace_only_files() {
            fs::write("test_whitespace.txt", "   \n\t\n  ").unwrap();

            let result = process_number_file("test_whitespace.txt");
            assert!(result.is_err());

            match result.unwrap_err() {
                FileProcessingError::EmptyFile => {},
                other => panic!("Expected EmptyFileError, got {:?}", other)
            }

            fs::remove_file("test_whitespace.txt").ok();
        }

        #[test]
        fn test_zero_validation() {
            fs::write("test_zero.txt", "10\n0\n30").unwrap();

            let result = process_number_file("test_zero.txt");
            assert!(result.is_err());

            match result.unwrap_err() {
                FileProcessingError::ValidationError(msg) => assert!(msg.contains("positive")),
                other => panic!("Expected ValidationError, got {:?}", other)
            }
        }

        #[test]
        fn test_error_display_messages() {
            let io_error = FileProcessingError::IoError(
                io::Error::new(io::ErrorKind::NotFound, "File not found")
            );
            assert!(format!("{}", io_error).contains("I/O error"));

            let validation_error = FileProcessingError::ValidationError("Test message".to_string());
            assert!(format!("{}", validation_error).contains("Validation error"));

            let empty_err = FileProcessingError::EmptyFile;
            assert_eq!(format!("{}", empty_err), "Empty file")

        }
    }
}

// =============================================================================
// MEDIUM CHALLENGES (4-6): Custom errors and advanced patterns
// =============================================================================

// MEDIUM CHALLENGE 4: Custom Error Types
// Goal: Create proper error enums with Display and Error traits
mod medium_challenge_4 {
    use super::*;

    #[derive(Debug)]
    enum CalculatorError {
        DivisionByZero,
        InvalidOperation(String),
        NumberTooLarge(f64),
        ParseError(String),
    }

    impl fmt::Display for CalculatorError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: Implement user-friendly error messages
            match self {
                CalculatorError::DivisionByZero => write!(f, "division by zero"),
                CalculatorError::InvalidOperation(op) => write!(f, "invalid operation: {}", op),
                CalculatorError::NumberTooLarge(n) => write!(f, "number is too large: {}", n),
                CalculatorError::ParseError(error) => write!(f, "parse error: {}", error),
            }
        }
    }

    impl std::error::Error for CalculatorError {}

    struct Calculator;

    impl Calculator {
        fn new() -> Self {
            Calculator
        }

        fn divide(&self, a: f64, b: f64) -> Result<f64, CalculatorError> {
            // TODO: Implement with proper error handling
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }

        fn parse_and_add(&self, a: &str, b: &str) -> Result<f64, CalculatorError> {
            // TODO: Parse both strings and add them
            // Handle parse errors appropriately
            let a_parsed = a.parse::<f64>();
            let b_parsed = b.parse::<f64>();

            match (a_parsed, b_parsed) {
                (Ok(a), Ok(b)) => Ok(a + b),
                (Err(a), _) => Err(CalculatorError::ParseError("Error parsing A".to_string())),
                (_, Err(b)) => Err(CalculatorError::ParseError("Error parsing B".to_string())),
            }
        }

        fn parse_and_add_improved(&self, a: &str, b: &str) -> Result<f64, CalculatorError> {
            let a_parsed = a.parse::<f64>()
                .map_err(|_| CalculatorError::ParseError(format!("Error parsing A: {}", a)))?;
            let b_parsed = b.parse::<f64>()
                .map_err(|_| CalculatorError::ParseError(format!("Error parsing B: {}", b)))?;

            Ok(a_parsed + b_parsed)
        }

        fn factorial(&self, n: u32) -> Result<u64, CalculatorError> {
            // TODO: Calculate factorial, but error if result would be too large
            // Consider n > 20 as too large
            if n > 20 {
                return Err(CalculatorError::NumberTooLarge(n as f64));
            }

            let mut count: u64 = 1;

            for i in 1..=n {
                count *= i as u64;
            }

            Ok(count)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculator_errors() {
            let calc = Calculator::new();

            // Division by zero
            assert!(matches!(calc.divide(10.0, 0.0), Err(CalculatorError::DivisionByZero)));

            // Parse error
            assert!(matches!(calc.parse_and_add("10", "abc"), Err(CalculatorError::ParseError(_))));

            // Number too large
            assert!(matches!(calc.factorial(25), Err(CalculatorError::NumberTooLarge(_))));

            // Success cases
            assert_eq!(calc.divide(10.0, 2.0).unwrap(), 5.0);
            assert_eq!(calc.parse_and_add("10", "20").unwrap(), 30.0);
        }
    }
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
