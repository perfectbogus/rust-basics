use std::cmp::max;
#[derive(Debug, PartialEq, Copy, Clone)]
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
        match self {
            Expression::Number(n) => { Ok(*n) }
            Expression::Operation(left, op, right) => {
                match op {
                    Operator::Add => { Ok(left.evaluate()? + right.evaluate()?) }
                    Operator::Subtract => { Ok(left.evaluate()? - right.evaluate()?) }
                    Operator::Multiply => { Ok(left.evaluate()? * right.evaluate()?) }
                    Operator::Divide => {
                        let right_value = right.evaluate()?;
                        if right_value == 0.0 {
                            Err(String::from("divide by zero"))
                        } else {
                            Ok(left.evaluate()? / right.evaluate()?)
                        }
                    }
                }
            }
            Expression::Negate(expr) => { Ok(-expr.evaluate()?) }
        }
    }

    fn simplify(&self) -> Expression {
        // TODO: Simplify the expression where possible
        // e.g., 5 + 0 = 5, x * 1 = x, x + (-y) = x - y
        match self {
            Expression::Number(n) => { Expression::Number(*n) }
            Expression::Negate(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Number(n) => { Expression::Number(-n)}
                    _ => Expression::Negate(Box::new(simplified))
                }
            },
            Expression::Operation(left, op, right) => {
                let left_simplified = left.simplify();
                let right_simplified = right.simplify();

                match (op, &left_simplified, &right_simplified) {
                    (Operator::Add, _, Expression::Number(n)) if *n == 0.0 => left_simplified,
                    (Operator::Add, Expression::Number(n), _) if *n == 0.0 => right_simplified,
                    (Operator::Multiply, _, Expression::Number(n)) if *n == 0.0 => Expression::Number(0.0),
                    (Operator::Multiply, Expression::Number(n), _) if *n == 0.0 => Expression::Number(0.0),
                    (Operator::Multiply, _, Expression::Number(n)) if *n == 1.0 => left_simplified,
                    (Operator::Multiply, Expression::Number(n), _) if *n == 1.0 => right_simplified,
                    (Operator::Divide, Expression::Number(n), _) if *n == 1.0 => left_simplified,
                    _ => Expression::Operation(
                        Box::new(left_simplified),
                        *op,
                        Box::new(right_simplified)
                    )
                }
            }

        }
    }

    fn height(&self) -> usize {
        // TODO: Calculate height of expression tree
        // Number nodes have height 1
        match self {
            Expression::Number(_) => { 1 }
            Expression::Operation(left, _, right) => {
                max(left.height(), right.height()) + 1
            }
            Expression::Negate(expr) => { expr.height() }
        }
    }

}

impl ExpressionBuilder {
    fn new() -> Self {
        // TODO: Initialize builder with empty stack
        Self { expression_stack: Vec::new() }
    }

    fn number(&mut self, value: f64) -> &mut Self {
        // TODO: Push a Number expression onto the stack
        let expr = Expression::Number(value);
        self.expression_stack.push(expr);
        self
    }

    fn operation(&mut self, op: Operator) -> Result<&mut Self, String> {
        // TODO: Pop two expression and combine them with the operator
        // return error if there aren't enough expression on stack
        let left = self.expression_stack.pop().ok_or(String::from("Expression stack is empty"))?;
        let right = self.expression_stack.pop().ok_or(String::from("Expression stack is empty"))?;

        match op {
            Operator::Add => {
                let result = Expression::Number(left.evaluate()? + right.evaluate()?);
                self.expression_stack.push(result);
                Ok(self)
            }
            Operator::Subtract => {
                let expr = Expression::Number(left.evaluate()? - right.evaluate()?);
                self.expression_stack.push(expr);
                Ok(self)
            }
            Operator::Multiply => {
                let expr = Expression::Number(left.evaluate()? * right.evaluate()?);
                self.expression_stack.push(expr);
                Ok(self)
            }
            Operator::Divide => {
                let expr = Expression::Number(left.evaluate()? / right.evaluate()?);
                self.expression_stack.push(expr);
                Ok(self)
            }
        }
    }

    fn negate(&mut self) -> Result<&mut Self, String> {
        // TODO: Pop one expression and negate it
        // Return error if stack is empty
        let expr = self.expression_stack.pop().ok_or(String::from("Expression stack is empty"))?;
        let value = expr.evaluate()?;
        let n_expr = Expression::Number(-value);
        self.expression_stack.push(n_expr);
        Ok(self)
    }

    fn build(&mut self) -> Result<Expression, String> {
        // TODO: Return final expression
        // Error if stack doesn't contain exactly one expression
        let expr = self.expression_stack.pop().ok_or(String::from("Expression stack is empty"))?;
        let value = expr.evaluate()?;
        Ok(Expression::Number(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expr = Expression::Operation(
            Box::new(Expression::Number(5.0)),
            Operator::Add,
            Box::new(Expression::Number(3.0))
        );

        assert_eq!(expr.evaluate().unwrap(), 8.0);

        let expr = Expression::Operation(
            Box::new(Expression::Number(5.0)),
            Operator::Subtract,
            Box::new(Expression::Number(3.0))
        );

        assert_eq!(expr.evaluate().unwrap(), 2.0);
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

    fn create_test_expression() -> Expression {
        // Test basic arithmetic: (5 + 3) * 2
        let expr = Expression::Operation(
            Box::new(Expression::Operation(
                Box::new(Expression::Number(5.0)),
                Operator::Add,
                Box::new(Expression::Number(3.0))
            )),
            Operator::Multiply,
            Box::new(Expression::Number(2.0))
        );
        expr
    }

    #[test]
    fn test_evaluate() {
        let expr = create_test_expression();

        assert_eq!(expr.evaluate().unwrap(), 16.0);
    }

    #[test]
    fn test_height() {
        // Test height of expression tree: (5 + 3) * 2
        let expr = create_test_expression();

        assert_eq!(expr.height(), 3);  // Level 1: *, Level 2: +, Level 3: numbers
    }

    #[test]
    fn test_expression_builder() {
        let mut builder = ExpressionBuilder::new();

        // Build: 5 + 3
        builder.number(5.0)
            .number(3.0)
            .operation(Operator::Add)
            .unwrap();

        let expr = builder.build().unwrap();
        assert_eq!(expr.evaluate().unwrap(), 8.0);
    }

    #[test]
    fn test_negate() {
        let mut builder = ExpressionBuilder::new();

        // Build: -5
        builder.number(5.0)
            .negate()
            .unwrap();

        let expr = builder.build().unwrap();
        assert_eq!(expr.evaluate().unwrap(), -5.0);
    }
}


fn main() {
    println!("Hello, world!");
}











































