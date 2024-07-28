use crate::expressions::expr::Expr;

pub mod ast_printer;
pub mod expr;

pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
}
