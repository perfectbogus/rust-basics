#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expression {
    Number(f64),
    Operation(Box<Expression>, Operator, Box<Expression>),
    Negate(Box<Expression>),
}

#[derive(Debug)]
struct ExpressionBuilder {
    expression_stack: Vec<Expression>,
}

impl Expression {
    fn evaluate(&self) -> Result<f64, String> {
        // TODO: Evaluate the expression
        // For Operation nodes, recursively evaluate left and right expressions
        // For Negate nodes, recursively evaluate and negate the result
        // For Number nodes, return the value
        unimplemented!()
    }

    fn simplify(&self) -> Expression {
        // TODO: Simplify the expression where possible
        // e.g., 5 + 0 = 5, x * 1 = x, x + (-y) = x - y
        unimplemented!()
    }

    fn height(&self) -> usize {
        // TODO: Calculate height of expression tree
        // Number nodes have height 1
        unimplemented!()
    }

}

impl ExpressionBuilder {
    fn new() -> Self {
        // TODO: Initialize builder with empty stack
        unimplemented!()
    }

    fn number(&mut self, value: f64) -> &mut self {
        // TODO: Push a Number expression onto the stack
        unimplemented!()
    }

    fn operation(&mut self, op: Operator) -> Result<&mut self, String> {
        // TODO: Pop two expression and combine them with the operator
        // return error if there aren't enough expression on stack
        unimplemented!()
    }

    fn negate(&mut self) -> Result<&mut self, String> {
        // TODO: Pop one expression and negate it
        // Return error if stack is empty
        unimplemented!()
    }

    fn build(&mut self) -> Result<Expression, String> {
        // TODO: Return final expression
        // Error if stack doesn't contain exactly one expression
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let expr = Expression::Operation(
            Box::new(Expression::Number(5.0)),
            Operator::Add,
            Box::new(Expression::Number(3.0))
        );

        assert_eq!(expr.evaluate().unwrap(), 8.0);
    }

    #[test]
    fn test_nested_expression() {
        // Build: (5 + 3) * 2
        let mut builder = ExpressionBuilder::new();
        builder.number(5.0)
            .number(3.0)
            .operation(Operator::Add).unwrap()
            .number(2.0)
            .operation(Operator::Multiply).unwrap();

        let expr = builder.build().unwrap();
        assert_eq!(expr.evaluate().unwrap(), 16.0);
    }

    #[test]
    fn test_negation() {
        let expr = Expression::Negate(
            Box::new(Expression::Number(5.0))
        );

        assert_eq!(expr.evaluate().unwrap(), -5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expression::Operation(
            Box::new(Expression::Number(5.0)),
            Operator::Divide,
            Box::new(Expression::Number(0.0))
        );

        assert!(expr.evaluate().is_err());
    }

    #[test]
    fn test_simplify() {
        // Test x + 0 = x
        let expr = Expression::Operation(
            Box::new(Expression::Number(5.0)),
            Operator::Add,
            Box::new(Expression::Number(0.0))
        );

        match expr.simplify() {
            Expression::Number(n) => assert_eq!(n, 5.0),
            _ => panic!("Expression wasn't simplified!")
        }
    }
}


fn main() {
    println!("Hello, world!");
}











































