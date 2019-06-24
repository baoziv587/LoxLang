use super::keywords::LoxKeywords;
use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    _source: &'a str,
    peeks: &'a str,
    tokens: Vec<Token>,
    start: usize,
    line: usize,
    inline_offset: usize,
    current: usize,
    count: usize,
}

pub fn is_digit(c: &str) -> bool {
    c.parse::<f64>().is_ok()
}

pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| char::is_alphanumeric(c) || c == '_')
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            _source: source,
            peeks: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            count: source.len(),
            inline_offset: 1,
        }
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        &self.peeks[start..end]
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.advance();
            self.scan_token();
        }
        self.create_token(
            TokenType::EOF,
            self.line,
            "\0".to_string(),
            self.inline_offset,
        );
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.count
    }

    fn scan_token(&mut self) {
        let c = self.current_char();
        match c {
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftParen),
            "}" => self.add_token(TokenType::RightParen),
            "," => self.add_token(TokenType::COMMA),
            "." => self.add_token(TokenType::DOT),
            "-" => self.add_token(TokenType::MINUS),
            "+" => self.add_token(TokenType::PLUS),
            ";" => self.add_token(TokenType::SEMICOLON),
            "*" => self.add_token(TokenType::STAR),
            "!" => {
                if self.match_str("=") {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            "=" => {
                if self.match_str("=") {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            ">" => {
                if self.match_str("=") {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            "<" => {
                if self.match_str("=") {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::LessEqual)
                }
            }
            "/" => {
                if self.match_str("/") {
                    self.scan_comment();
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            // resolve string literals*
            "\"" => self.scan_string(),
            // numeric char
            c if is_digit(c) => self.scan_number(),
            c if is_alphanumeric(c) => self.scan_identifier(),
            // ignore whitespace
            " " | "\r" | "\t" => (),
            "\n" => self.scan_new_line(),
            _ => (),
        }
    }

    fn scan_string(&mut self) {
        // consume the '"'
        self.advance();

        while !self.is_at_end() {
            let s = self.peek(None).unwrap();
            if s == "\"" {
                break;
            }
            if s == "\n" {
                self.scan_new_line();
            }
            self.advance();
        }

        if self.is_at_end() {
            return println!("Unterminated string ");
        }

        let end = self.current - 1;
        let start = self.start + 1;
        let content = self.slice(start, end);
        self.create_token(
            TokenType::STRING,
            self.line,
            content.to_string(),
            self.inline_offset,
        );
    }

    fn scan_comment(&mut self) {
        while !self.is_at_end() && self.peek(None).unwrap() != "\n" {
            self.advance();
        }
        self.scan_new_line();
    }

    fn scan_new_line(&mut self) {
        self.line += 1;
        self.inline_offset = 0;
    }

    fn scan_number(&mut self) {
        while !self.is_at_end() && is_digit(self.peek_next().unwrap()) {
            self.advance();
        }

        if !self.is_at_end() && self.match_str(".") && is_digit(self.peek_next().unwrap()) {
            // consume "."
            self.advance();
            while is_digit(self.peek_next().unwrap()) {
                self.advance();
            }
        }

        // meet non-numeric char
        let value = self.slice(self.start, self.current);
        self.create_token(
            TokenType::NUMBER,
            self.line,
            value.to_string(),
            self.inline_offset,
        );
    }

    fn scan_identifier(&mut self) {
        while is_alphanumeric(self.peek_next().unwrap()) {
            self.advance();
        }

        let val = self.slice(self.start, self.current);
        let keyword_type = LoxKeywords.get(val);

        if keyword_type.is_some() {
            self.add_token(*keyword_type.unwrap());
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }

    fn match_str(&mut self, expected: &str) -> bool {
        let next = self.peek_next().unwrap();
        if next != expected {
            return false;
        }
        self.advance();
        return true;
    }

    fn current_char(&mut self) -> &str {
        self.peek(None).unwrap()
    }

    fn advance(&mut self) -> Option<&str> {
        self.current += 1;
        self.inline_offset += 1;
        self.peek(None)
    }

    fn peek(&mut self, n: Option<usize>) -> Option<&str> {
        let end = if n.is_some() {
            n.unwrap()
        } else {
            self.current
        };
        let start = if end <= 0 { 0 } else { end - 1 };
        if end >= self.count + 1 {
            return Some("\0");
        }
        let c: &str = self.slice(start, end);
        Some(&c)
    }

    fn peek_next(&mut self) -> Option<&str> {
        self.peek(Some(self.current + 1))
    }

    fn create_token(&mut self, t: TokenType, line: usize, val: String, pos: usize) -> Token {
        let t = Token::new(t, line, val, pos);
        self.tokens.push(t.clone());
        t
    }

    fn add_token(&mut self, tok_type: TokenType) {
        let val = self.slice(self.start, self.current).to_string();
        self.create_token(tok_type, self.line, val, self.inline_offset);
    }
}

#[cfg(test)]
mod tests {
    pub fn init() {
        #[cfg(not(feature = "pretty-env-logger"))]
        env_logger::init();
        #[cfg(feature = "pretty-env-logger")]
        pretty_env_logger::init();
    }
}

use std::fs::File;
use std::io::Read;
#[test]
fn test_scan_identifier() {
    tests::init();
    let src = String::from("People123123{};");
    let mut scanner = Scanner::new(&src);
    scanner.scan_identifier();
    let mut it = scanner.tokens.iter();

    assert_eq!(it.next().unwrap().val, "People123123");

    let src = String::from("class People_123123{ var 2=3};");
    let mut scanner = Scanner::new(&src);
    scanner.scan_tokens();
    let mut it = scanner.tokens.iter();
    assert_eq!(it.next().unwrap().val, "class");
    assert_eq!(it.next().unwrap().val, "People_123123");
    assert_eq!(it.next().unwrap().val, "{");
    assert_eq!(it.next().unwrap().val, "var");
    assert_eq!(it.next().unwrap().val, "2");
    assert_eq!(it.next().unwrap().val, "=");
    assert_eq!(it.next().unwrap().val, "3");
    assert_eq!(it.next().unwrap().val, "}");
    assert_eq!(it.next().unwrap().val, ";");

    let src = String::from("class People-123123{ var 2=3};");
    let mut scanner = Scanner::new(&src);
    scanner.scan_tokens();
    let mut it = scanner.tokens.iter();
    assert_eq!(it.next().unwrap().val, "class");
    assert_eq!(it.next().unwrap().val, "People");
    assert_eq!(it.next().unwrap().val, "-");
    assert_eq!(it.next().unwrap().val, "123123");
    assert_eq!(it.next().unwrap().val, "{");
    assert_eq!(it.next().unwrap().val, "var");
    assert_eq!(it.next().unwrap().val, "2");
    assert_eq!(it.next().unwrap().val, "=");
    assert_eq!(it.next().unwrap().val, "3");
    assert_eq!(it.next().unwrap().val, "}");
    assert_eq!(it.next().unwrap().val, ";");
}

#[test]
fn test_scan_number() {
    let src = String::from("23333");
    let mut scanner = Scanner::new(&src);
    scanner.scan_number();
    assert_eq!(scanner.tokens.len(), 1);
    let mut it = scanner.tokens.iter();
    assert_eq!(it.next().unwrap().val, "23333");

    let src = String::from("2.3333--3333");
    let mut scanner = Scanner::new(&src);
    scanner.scan_tokens();
    let mut it = scanner.tokens.iter();
    assert_eq!(it.next().unwrap().val, "2.3333");
    assert_eq!(it.next().unwrap().val, "-");
    assert_eq!(it.next().unwrap().val, "-");
    assert_eq!(it.next().unwrap().val, "3333");
}

#[test]
fn test_advance() {
    let mut src = String::new();
    let _f = File::open("./src/fixtures/ex1.lox").and_then(|mut f| f.read_to_string(&mut src));
    let mut scanner = Scanner::new(&src);

    scanner.advance();
    assert_eq!(Some("c"), Some(scanner.current_char()));
    scanner.advance();
    assert_eq!(Some("l"), Some(scanner.current_char()));
    scanner.advance();
    assert_eq!(Some("a"), Some(scanner.current_char()));
    scanner.advance();
    assert_eq!(Some("s"), Some(scanner.current_char()));
    scanner.advance();
    assert_eq!(Some("s"), Some(scanner.current_char()));
}

#[test]
fn test_scan_tokens() {
    let mut src = String::new();
    let _f = File::open("./src/fixtures/ex1.lox").and_then(|mut f| f.read_to_string(&mut src));
    let mut scanner = Scanner::new(&src);
    scanner.scan_tokens();
    for t in scanner.tokens.iter() {
        info!("{}", t.to_string());
    }
}
