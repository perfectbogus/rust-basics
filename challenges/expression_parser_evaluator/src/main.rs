fn main() {
    let expr = Expr::multiply(
        Expr::add(Expr::number(5.0), Expr::number(3.0)),
        Expr::subtract(Expr::number(10.0), Expr::number(6.0))
    );

    let result = expr.evaluate();
    println!("result: {}", result);

    let str = expr.to_string();
    println!("str: {}", str);
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn number(n: f64) -> Self {
        Expr::Number(n)
    }

    fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    fn subtract(left: Expr, right: Expr) -> Self {
        Expr::Subtract(Box::new(left), Box::new(right))
    }

    fn multiply(left: Expr, right: Expr) -> Self {
        Expr::Multiply(Box::new(left), Box::new(right))
    }

    fn divide(left: Expr, right: Expr) -> Self {
        Expr::Divide(Box::new(left), Box::new(right))
    }

    fn evaluate(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(left, right) => left.evaluate() + right.evaluate(),
            Expr::Subtract(left, right) => left.evaluate() - right.evaluate(),
            Expr::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Expr::Divide(left, right) => left.evaluate() / right.evaluate()
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Add(left, right) => format!("({} + {})", left.to_string(), right.to_string()),
            Expr::Subtract(left, right) => format!("({} - {})", left.to_string(), right.to_string()),
            Expr::Multiply(left, right) => format!("({} * {})", left.to_string(), right.to_string()),
            Expr::Divide(left, right) => format!("({} / {})", left.to_string(), right.to_string())
        }
    }
}

