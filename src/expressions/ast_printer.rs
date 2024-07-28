use std::fmt::Write;

use crate::expressions::expr::Expr;
use crate::expressions::Visitor;
use crate::token::Token;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&self, op: &str, exprs: Vec<&Expr>) -> String {
        let mut result = String::new();

        write!(&mut result, "({op}").unwrap();
        for expr in exprs {
            write!(&mut result, " ").unwrap();
            write!(&mut result, "{}", expr.accept(self)).unwrap();
        }
        write!(&mut result, ")").unwrap();

        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.parenthesize(&op.lexeme, vec![lhs, rhs]),
            Expr::Grouping(exp) => self.parenthesize("group", vec![exp]),
            Expr::Literal(literal) => literal.to_string(),
            Expr::Unary(op, rhs) => self.parenthesize(&op.lexeme, vec![rhs]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_printer_binary() {
        // -123 * (45.67)

        let expr = Expr::Binary(
            Box::new(Expr::Unary(Token::minus(1), Box::new(Expr::number(123_f64)))),
            Token::star(1),
            Box::new(Expr::Grouping(Box::new(Expr::number(45.67_f64)))),
        );

        let printer = AstPrinter;

        assert_eq!("(* (- 123) (group 45.67))", printer.print(&expr));
    }
}
