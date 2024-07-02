mod ast {
    pub enum Stmt  {
        Expr(Expr),
        Let(Name, Expr)
    }

    pub struct Name(String);

    pub enum Expr {
        Literal(i64),
        Sum(Box<Expr>, Box<Expr>),
        Mult(Box<Expr>, Box<Expr>)
    }
}

mod visit {
    use super::ast::*;
    pub trait Visitor<T> {
        fn visit_name(&mut self, name: &Name) -> T;
        fn visit_stmt(&mut self, expr: &Stmt) -> T;
        fn visit_expr(&mut self, expr: &Expr) -> T;
    }
}

use std::collections::HashMap;

use visit::Visitor;

struct Interpreter {
    bindings: HashMap<String, i64>
}

impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, name: &ast::Name) -> i64 {
        todo!()
    }

    fn visit_stmt(&mut self, expr: &ast::Stmt) -> i64 {
        todo!()
    }

    fn visit_expr(&mut self, expr: &ast::Expr) -> i64 {
        todo!()
    }
}


fn main () {
    
}
