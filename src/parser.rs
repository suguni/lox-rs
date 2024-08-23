use std::mem;
use crate::expressions::expr::Expr;
use crate::token::{Token, TokenLiteral, TokenType};

pub struct Parse {
    tokens: Vec<Token>,
    current: usize,
}

impl Parse {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::False]) {
            return Expr::Literal(TokenLiteral::False);
        }

        if self.match_tokens(vec![TokenType::True]) {
            return Expr::Literal(TokenLiteral::True);
        }

        if self.match_tokens(vec![TokenType::Nil]) {
            return Expr::Literal(TokenLiteral::Nil);
        }

        if self.match_tokens(vec![TokenType::String, TokenType::Number]) {
            return Expr::Literal(self.previous().clone().literal.unwrap());
        }

        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return  Expr::Grouping(Box::new(expr));
        }

        panic!("Expect primary token")
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {

    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }
        false
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() || self.peek().token_type == *token_type
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

