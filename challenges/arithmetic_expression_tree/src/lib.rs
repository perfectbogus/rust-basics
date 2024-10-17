/*
Your task is to implement an arithmetic expression tree that can
represent and evaluate simple mathematical expressions.
The tree should support addition, subtraction, multiplication,
and division operations, as well as integer values.
 */

enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn value(v: i32) -> Box<Expr>{
        Box::new(Expr::Value(v))
    }

    fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Add(left, right))
    }

    fn subtract(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Subtract(left, right))
    }

    fn multiply(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Multiply(left, right))
    }

    fn divide(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Divide(left, right))
    }

    fn evaluate(&self) -> Result<i32, String> {
        match self {
            Expr::Value(v) => Ok(*v),
            Expr::Add(left, right) => {
                let l = left.evaluate()?;
                let r = right.evaluate()?;
                Ok(l + r)
            },
            Expr::Subtract(left, right) => {
                let l = left.evaluate()?;
                let r = right.evaluate()?;
                Ok(l - r)
            },
            Expr::Multiply(left, right) => {
                let l = left.evaluate()?;
                let r = right.evaluate()?;
                Ok(l * r)
            },
            Expr::Divide(left, right) => {
                let l = left.evaluate()?;
                let r = right.evaluate()?;
                if r == 0 {
                    Err("Division by Zero".to_string())
                } else {
                    Ok(l / r)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let expr = Expr::add(Expr::value(5), Expr::value(3));
        assert_eq!(expr.evaluate(), Ok(8));
    }

    #[test]
    fn test_nested_expression() {
        let expr = Expr::multiply(
            Expr::add(Expr::value(2), Expr::value(3)),
            Expr::subtract(Expr::value(5), Expr::value(1))
        );
        assert_eq!(expr.evaluate(), Ok(20));
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expr::divide(Expr::value(10), Expr::value(0));
        assert!(expr.evaluate().is_err());
    }

    // TODO: Add more test cases
}