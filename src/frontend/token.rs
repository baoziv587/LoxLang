#![allow(dead_code)]

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // ONE OR TWO CHARACTER TOKENS
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literials.
    IDENTIFIER,
    STRING,
    NUMBER,

    // KEYWORDS.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl TokenType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub tok_type: TokenType,
    pub line: usize,
    pub val: String,
    pub pos: usize,
}

impl Token {
    pub fn new(t_type: TokenType, line: usize, val: String, pos: usize) -> Token {
        Token {
            tok_type: t_type,
            line,
            val,
            pos,
        }
    }

    pub fn to_string(&self) -> String {
        return self.tok_type.to_string() + " " + self.val.as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_token() {
        init();
        let line_num = 32;
        let t = Token::new(TokenType::AND, line_num, String::from("&"), 5);
        assert_eq!("&", t.val);
        assert_eq!(32, line_num);
        assert_eq!(TokenType::AND, t.tok_type);
    }

    #[test]
    fn test_token_to_string() {
        let line_num = 32;
        let t = Token::new(TokenType::AND, line_num, String::from("&"), 5);
        assert_eq!("AND &", t.to_string());
    }
}
