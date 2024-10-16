
#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Paren(Box<Expr>),
}

fn tokenizer(input: &str) -> Vec<Token> {
    input.split_whitespace()
        .map(|s| match s {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            _ => Token::Number(s.parse().unwrap())
        })
        .collect()
}

fn parse(tokens: &[Token]) -> Option<Expr> {
    None
}

fn evaluate(expr: &Expr) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_evaluate() {
        let cases = vec![
            ("5", 5.0),
            ("3 + 4", 7.0),
            ("3 - 4", -1.0),
            ("3 * 4", 12.0),
            ("12 / 4", 3.0),
            ("( 3 + 4 ) * 2", 14.0),
            ("3 + 4 * 2", 11.0),
            ("( 7 + 3 ) * ( 5 - 2 )", 30.0),
        ];

        for (input, expected) in cases {
            let tokens = tokenizer(input);
            let expr = parse(&tokens).unwrap();
            let result = evaluate(&expr);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}