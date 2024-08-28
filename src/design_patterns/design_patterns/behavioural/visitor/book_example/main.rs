

mod ast {
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>)
    }

    pub enum Name {
        Value(String)
    }

    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr)
    }
}

mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_expr(&mut self, e: &Expr) -> T;
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
    }
}

use crate::ast::{Expr, Name, Stmt};
use crate::visit::Visitor;
struct Interpreter;

impl Visitor<i64> for Interpreter {
    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs)
        }
    }

    fn visit_name(&mut self, n: &Name) -> i64 {
        unimplemented!()
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!()
        }
    }
}

fn main() {

}