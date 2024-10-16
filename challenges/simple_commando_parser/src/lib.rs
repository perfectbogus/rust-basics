#[derive(Debug, PartialEq)]
enum Command {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Greet(String),
    Quit,
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts = input.split_whitespace();

    match parts.next() {
        Some("add") => {
            let x = parts.next().ok_or("Missing first number")?.parse().map_err(|_| "Invalid first number")?;
            let y = parts.next().ok_or("Missing second number")?.parse().map_err(|_| "Invalid second number")?;
            Ok(Command::Add(x, y))
        },
        Some("subtract") => {
            let x = parts.next().ok_or("Missing 1st number")?.parse().map_err(|_| "Invalid 1st number")?;
            let y = parts.next().ok_or("Missing second number")?.parse().map_err(|_| "Invalid second number")?;
            Ok(Command::Subtract(x, y))
        },
        Some("multiply") => {
            let x = parts.next().ok_or("Missing 1st number")?.parse().map_err(|_| "Invalid 1st number")?;
            let y = parts.next().ok_or("Missing second number")?.parse().map_err(|_| "Invalid second number")?;
            Ok(Command::Multiply(x, y))
        },
        Some("divide") => {
            let x = parts.next().ok_or("Missing 1st number")?.parse().map_err(|_| "Invalid 1st number")?;
            let y = parts.next().ok_or("Missing second number")?.parse().map_err(|_| "Invalid second number")?;
            Ok(Command::Divide(x, y))
        },
        Some("greet") => {
            let name = parts.next().ok_or("Missing name")?;
            Ok(Command::Greet(name.to_string()))
        },
        Some("quit") => {
            Ok(Command::Quit)
        }
        Some(cmd) => Err(format!("Unknown command: {}", cmd)),
        None => Err("Empty input".to_string()),
    }
}

fn execute_command(command: Command) {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add() {
        assert_eq!(parse_command("add 5 3"), Ok(Command::Add(5.0, 3.0)));
    }

    #[test]
    fn test_parse_subtract() {
        assert_eq!(parse_command("subtract 10 4"), Ok(Command::Subtract(10.0, 4.0)));
    }

    #[test]
    fn test_parse_multiply() {
        assert_eq!(parse_command("multiply 2.5 3"), Ok(Command::Multiply(2.5, 3.0)));
    }

    #[test]
    fn test_parse_divide() {
        assert_eq!(parse_command("divide 15 3"), Ok(Command::Divide(15.0, 3.0)));
    }

    #[test]
    fn test_parse_greet() {
        assert_eq!(parse_command("greet Alice"), Ok(Command::Greet("Alice".to_string())));
    }

    #[test]
    fn test_parse_quit() {
        assert_eq!(parse_command("quit"), Ok(Command::Quit));
    }

    #[test]
    fn test_parse_invalid_command() {
        assert!(parse_command("invalid command").is_err());
    }

    // Additional test cases for error handling
    #[test]
    fn test_parse_add_missing_number() {
        assert!(parse_command("add 5").is_err());
    }

    #[test]
    fn test_parse_add_invalid_number() {
        assert!(parse_command("add 5 abc").is_err());
    }

    #[test]
    fn test_parse_empty_input() {
        assert!(parse_command("").is_err());
    }
}