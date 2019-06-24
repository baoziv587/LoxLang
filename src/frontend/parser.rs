use crate::frontend::ast::expr::{BinaryExpr, Boxer, Expr};
use crate::frontend::token::{Token, TokenType};
use crate::result::{Error, Result};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expressions(&mut self) -> Result<Expr> {
        self.equality()
    }

    /// equality ->  comparison ( "=" | "==") comparison
    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        let types = vec![TokenType::BangEqual, TokenType::EqualEqual];

        while self.match_type(&types) {
            let op = self.previous().unwrap().clone();
            let right = self.comparison()?;
            let binary_expr = BinaryExpr::new(expr.boxed(), op, right.boxed()).boxed();
            expr = Expr::BinaryExpr(binary_expr)
        }
        Ok(expr)
    }

    fn match_type(&mut self, types: &Vec<TokenType>) -> bool {
        for t in types.iter() {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expect_next(&mut self, types: &Vec<TokenType>) -> Result<&Token> {
        if self.match_type(types) {
            return Ok(self.peek().unwrap());
        }

        Err(self.peek_error())
    }

    fn peek_error(&mut self) -> Error {
        // peek for EOF and unexpected tokens
        let pk = self.peek().cloned();

        if pk.is_none() {
            return self.eof();
        }

        if let Some(tkn) = pk {
            return self.unexpected(&tkn);
        }
        unreachable!()
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.advance();
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().unwrap().tok_type == *t
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // comparison → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
    fn comparison(&mut self) -> Result<Expr> {
        let types = vec![
            TokenType::GREATER,
            TokenType::GreaterEqual,
            TokenType::LESS,
            TokenType::LessEqual,
        ];

        let mut expr = self.addition()?;
        while self.match_type(&types) {
            let op = self.previous().unwrap().clone();
            let right = self.addition()?;
            let e = BinaryExpr::new(expr.boxed(), op, right.boxed()).boxed();
            expr = Expr::BinaryExpr(e);
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr> {
        let mut expr = self.mutilplication()?;
        let types = vec![TokenType::MINUS, TokenType::PLUS];

        while self.match_type(&types) {
            let operator = self.previous().unwrap().clone();
            let right = self.mutilplication()?.boxed();
            let binary_expr = BinaryExpr::new(expr.boxed(), operator, right).boxed();
            expr = Expr::BinaryExpr(binary_expr);
        }
        Ok(expr)
    }

    fn mutilplication(&mut self) -> Result<Expr> {
        unimplemented!()
    }

    /// unary → ( "!" | "-" ) unary
    ///         | primary ;
    fn unray(&mut self) -> Result<Expr> {
        if self.match_type(&vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().unwrap().clone();
            let right = self.unray()?;
            return Ok(Expr::Unary(op, right.boxed()));
        }
        self.primary()
    }

    /// primary → NUMBER | STRING | "false" | "true" | "nil"
    ///                  | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr> {
        let t = self.peek().unwrap().clone();
        match t.tok_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => {
                self.advance();
                Ok(Expr::Literal(t.val.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expressions()?;
                let _ = self.expect_next(&vec![TokenType::RightParen]);
                Ok(Expr::Grouping(expr.boxed()))
            }
            _ => unreachable!(),
        }
    }

    fn unexpected(&mut self, t: &Token) -> Error {
        Error::Parse(
            t.line as u64,
            t.pos as u64,
            "unexpected token".to_string(),
            t.to_string(),
        )
    }

    fn eof(&self) -> Error {
        Error::Parse(0, 0, "".to_string(), "unexpected EOF".to_string())
    }
}
