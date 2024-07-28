use crate::expressions::Visitor;
use crate::token::{Token, TokenLiteral};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(TokenLiteral),
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn accept<V, T>(&self, visitor: &V) -> T
    where
        V: Visitor<T>,
    {
        visitor.visit_expr(self)
    }

    pub fn number(num: f64) -> Self {
        Expr::Literal(TokenLiteral::Number(num))
    }
}
