use crate::error::error;
use crate::token::{Token, TokenLiteral, TokenType};

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut scanner = Scanner {
            source: source.chars().collect::<Vec<char>>(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        scanner
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::eof(self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_sym(TokenType::LeftParen),
            ')' => self.add_token_sym(TokenType::RightParen),
            '{' => self.add_token_sym(TokenType::LeftBrace),
            '}' => self.add_token_sym(TokenType::RightBrace),
            ',' => self.add_token_sym(TokenType::Comma),
            '.' => self.add_token_sym(TokenType::Dot),
            '-' => self.add_token_sym(TokenType::Minus),
            '+' => self.add_token_sym(TokenType::Plus),
            ';' => self.add_token_sym(TokenType::Semicolon),
            '*' => self.add_token_sym(TokenType::Star),
            '!' => {
                let token_type = self.match_token('=', TokenType::BangEqual, TokenType::Bang);
                self.add_token_sym(token_type)
            }
            '=' => {
                let token_type = self.match_token('=', TokenType::EqualEqual, TokenType::Equal);
                self.add_token_sym(token_type)
            }
            '<' => {
                let token_type = self.match_token('=', TokenType::LessEqual, TokenType::Less);
                self.add_token_sym(token_type);
            }
            '>' => {
                let token_type = self.match_token('=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_token_sym(token_type);
            }
            '/' => self.slash(),
            '"' => self.string(),
            '\n' => self.line += 1,
            c if c != '\n' && c.is_whitespace() => {}

            c if is_digit(c) => self.number(),

            c if is_alpha(c) => self.identifier(),
            _ => error(self.line, "Unexpected character."),
        }
    }

    fn slash(&mut self) {
        match self.peek() {
            '/' => self.line_comment(),
            '*' => { self.block_comment(); },
            _ => self.add_token_sym(TokenType::Slash),
        }
    }

    fn line_comment(&mut self) {
        self.advance();
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn block_comment(&mut self) -> bool {
        loop {
            let c = self.advance();
            if c == '/' {
                if self.peek() == '*' {
                    if !self.block_comment() {
                        break false;
                    }
                }
            } else if c == '*' {
                if self.peek() == '/' {
                    self.advance();
                    break true;
                }
            } else if c == '\n' {
                self.line += 1;
            } else if self.is_at_end() {
                error(self.line, "Unterminated block comment");
                break false;
            }
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].iter().collect::<String>();
        let token_type = TokenType::keyword(&text).unwrap_or(TokenType::Identifier);
        self.add_token_sym(token_type);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
        }

        while is_digit(self.peek()) {
            self.advance();
        }

        self.add_token_number();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        self.add_token_string();
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_token(&mut self, expected_char: char,
                   expected_token_type: TokenType,
                   otherwise_token_type: TokenType) -> TokenType {
        if self.match_char(expected_char) {
            expected_token_type
        } else {
            otherwise_token_type
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let pos = self.current;
        self.current += 1;
        self.source[pos]
    }

    fn add_token_sym(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].iter().collect::<String>();
        self.add_token(token_type, text, None)
    }

    fn add_token_string(&mut self) {
        let text = self.source[self.start..self.current].iter().collect::<String>();
        let value = self.source[self.start + 1..self.current - 1].iter().collect::<String>();
        self.add_token(TokenType::String, text, Some(TokenLiteral::String(value)))
    }

    fn add_token_number(&mut self) {
        let text = self.source[self.start..self.current].iter().collect::<String>();
        match text.parse() {
            Ok(num) => self.add_token(TokenType::Number, text, Some(TokenLiteral::Number(num))),
            Err(_) => error(self.line, "Failed to parse number literal"),
        }
    }

    fn add_token(&mut self, token_type: TokenType, text: String, literal: Option<TokenLiteral>) {
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}


#[cfg(test)]
mod tests {
    use crate::error::has_error;

    use super::*;

    #[test]
    fn test_eof() {
        let scanner = Scanner::new("");

        assert!(!has_error());

        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Eof, String::from(""), None, 1));
    }

    #[test]
    fn test_single_char_op() {
        let scanner = Scanner::new("+");

        assert!(!has_error());

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Plus, String::from("+"), None, 1));
        assert_eq!(scanner.tokens[1],
                   Token::new(TokenType::Eof, String::from(""), None, 1));
    }

    #[test]
    fn test_two_char_op() {
        let scanner = Scanner::new("!=");

        assert!(!has_error());

        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::BangEqual, String::from("!="), None, 1));
    }

    #[test]
    fn test_comment() {
        let scanner = Scanner::new("// comment line");

        dbg!(&scanner.tokens);

        assert!(!has_error());
        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_single_line_string_literal() {
        let scanner = Scanner::new("\"hello world\"");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::String, String::from("\"hello world\""),
                              Some(TokenLiteral::String(String::from("hello world"))), 1));
    }

    #[test]
    fn test_multi_line_string_literal() {
        let scanner = Scanner::new("\"hello\n\nworld\"");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::String, String::from("\"hello\n\nworld\""),
                              Some(TokenLiteral::String(String::from("hello\n\nworld"))), 3));
    }

    #[test]
    fn test_uncompleted_string() {
        let _ = Scanner::new("\"hello");

        assert!(has_error());
    }

    #[test]
    fn test_int_number_literal() {
        let scanner = Scanner::new("1234");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Number, String::from("1234"),
                              Some(TokenLiteral::Number(1234.0)), 1));
    }

    #[test]
    fn test_float_number_literal() {
        let scanner = Scanner::new("1234.56");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Number, String::from("1234.56"),
                              Some(TokenLiteral::Number(1234.56)), 1));
    }

    #[test]
    fn test_keyword() {
        let scanner = Scanner::new("class");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Class, String::from("class"), None, 1));
    }

    #[test]
    fn test_identifier() {
        let scanner = Scanner::new("classic");

        assert!(!has_error());
        assert_eq!(scanner.tokens[0],
                   Token::new(TokenType::Identifier, String::from("classic"), None, 1));
    }

    #[test]
    fn test_statement() {
        let scanner = Scanner::new("var language = \"lox\";\nvar a = 10.2;\n");

        assert!(!has_error());
        assert_eq!(scanner.tokens, vec![
            Token::new(TokenType::Var, String::from("var"), None, 1),
            Token::new(TokenType::Identifier, String::from("language"), None, 1),
            Token::new(TokenType::Equal, String::from("="), None, 1),
            Token::new(TokenType::String, String::from("\"lox\""), Some(TokenLiteral::String(String::from("lox"))), 1),
            Token::new(TokenType::Semicolon, String::from(";"), None, 1),

            Token::new(TokenType::Var, String::from("var"), None, 2),
            Token::new(TokenType::Identifier, String::from("a"), None, 2),
            Token::new(TokenType::Equal, String::from("="), None, 2),
            Token::new(TokenType::Number, String::from("10.2"), Some(TokenLiteral::Number(10.2)), 2),
            Token::new(TokenType::Semicolon, String::from(";"), None, 2),

            Token::new(TokenType::Eof, String::from(""), None, 3),
        ]);
    }

    #[test]
    fn test_block_comment() {
        let scanner = Scanner::new("/* comment */");

        assert!(!has_error());
        assert_eq!(scanner.tokens, vec![Token::new(TokenType::Eof, String::from(""), None, 1),]);
    }

    #[test]
    fn test_block_comment_in_code() {
        let scanner = Scanner::new("var x = \"lox\";\n/* comment */var a = 10.2;\n");

        assert!(!has_error());
        assert_eq!(scanner.tokens, vec![
            Token::new(TokenType::Var, String::from("var"), None, 1),
            Token::new(TokenType::Identifier, String::from("x"), None, 1),
            Token::new(TokenType::Equal, String::from("="), None, 1),
            Token::new(TokenType::String, String::from("\"lox\""), Some(TokenLiteral::String(String::from("lox"))), 1),
            Token::new(TokenType::Semicolon, String::from(";"), None, 1),

            Token::new(TokenType::Var, String::from("var"), None, 2),
            Token::new(TokenType::Identifier, String::from("a"), None, 2),
            Token::new(TokenType::Equal, String::from("="), None, 2),
            Token::new(TokenType::Number, String::from("10.2"), Some(TokenLiteral::Number(10.2)), 2),
            Token::new(TokenType::Semicolon, String::from(";"), None, 2),

            Token::new(TokenType::Eof, String::from(""), None, 3),
        ]);
    }

    #[test]
    fn test_nested_block_comment() {
        let scanner = Scanner::new("var a = \"a\";\n/* com /* ok */ ment */var a = 10.2;\n");

        assert!(!has_error());
        assert_eq!(scanner.tokens, vec![
            Token::new(TokenType::Var, String::from("var"), None, 1),
            Token::new(TokenType::Identifier, String::from("a"), None, 1),
            Token::new(TokenType::Equal, String::from("="), None, 1),
            Token::new(TokenType::String, String::from("\"a\""), Some(TokenLiteral::String(String::from("a"))), 1),
            Token::new(TokenType::Semicolon, String::from(";"), None, 1),

            Token::new(TokenType::Var, String::from("var"), None, 2),
            Token::new(TokenType::Identifier, String::from("a"), None, 2),
            Token::new(TokenType::Equal, String::from("="), None, 2),
            Token::new(TokenType::Number, String::from("10.2"), Some(TokenLiteral::Number(10.2)), 2),
            Token::new(TokenType::Semicolon, String::from(";"), None, 2),

            Token::new(TokenType::Eof, String::from(""), None, 3),
        ]);
    }
}