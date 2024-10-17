/*
Let's create a simple calculator that can handle basic arithmetic
operations: addition, subtraction, multiplication, and division.
 We'll parse simple expressions without worrying about parentheses or complex precedence rules.
 */

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn tokenize(input: &str) -> Vec<Token> {
    input.split_whitespace()
        .map(|s| match s {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            _ => Token::Number(s.parse().unwrap()),
        })
        .collect()
}

fn calculate(tokens: &[Token]) -> Option<f64> {
    let mut result = 0.0;
    let mut current_op = Token::Plus;

    for token in tokens {
        match token {
            Token::Number(n) => {
                match current_op {
                    Token::Plus => result += n,
                    Token::Minus => result -= n,
                    Token::Multiply => result *= n,
                    Token::Divide => {
                        if *n == 0.0 {
                            return None; // Division by zero
                        }
                        result /= n;
                    },
                    _ => return None, // This shouldn't happen
                }
            },
            op => current_op = op.clone(),
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let tokens = tokenize("5 + 3");
        assert_eq!(calculate(&tokens), Some(8.0));
    }

    #[test]
    fn test_simple_subtraction() {
        let tokens = tokenize("10 - 4");
        assert_eq!(calculate(&tokens), Some(6.0));
    }

    #[test]
    fn test_simple_multiplication() {
        let tokens = tokenize("3 * 4");
        assert_eq!(calculate(&tokens), Some(12.0));
    }

    #[test]
    fn test_simple_division() {
        let tokens = tokenize("15 / 3");
        assert_eq!(calculate(&tokens), Some(5.0));
    }

    #[test]
    fn test_complex_expression() {
        let tokens = tokenize("3 + 4 * 2 - 6 / 2");
        assert_eq!(calculate(&tokens), Some(10.0));
    }
}