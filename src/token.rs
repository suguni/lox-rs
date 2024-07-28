use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn keyword(text: &str) -> Option<Self> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLiteral::String(s) => write!(f, "{}", s),
            TokenLiteral::Number(n) => write!(f, "{}", n),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Option<TokenLiteral>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<TokenLiteral>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line,
        }
    }

    pub fn minus(line: usize) -> Self {
        Token {
            token_type: TokenType::Minus,
            lexeme: String::from("-"),
            literal: None,
            line,
        }
    }

    pub fn star(line: usize) -> Self {
        Token {
            token_type: TokenType::Star,
            lexeme: String::from("*"),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_to_string() {
        let token = Token {
            token_type: TokenType::Class,
            lexeme: String::from("class"),
            literal: None,
            line: 10,
        };

        assert_eq!(token.to_string(), String::from("CLASS class 10"));
    }
}