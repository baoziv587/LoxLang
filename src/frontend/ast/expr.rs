// use super::super::token::TokenType;

use crate::frontend::token::Token;

pub trait Boxer {
    // add code here
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Noop,
    BinaryExpr(Box<BinaryExpr>),
    Unary(Token, Box<Expr>),
    Literal(String),
    Grouping(Box<Expr>),
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct BinaryExpr {
    op: Token,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Boxer for Expr {}

impl BinaryExpr {
    pub fn new(lhs: Box<Expr>, op: Token, rhs: Box<Expr>) -> Self {
        BinaryExpr { op, lhs, rhs }
    }
}

impl Boxer for BinaryExpr {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Slash,
    Star,
    Equal,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl BinaryOperator {
    pub fn to_string(&self) -> &str {
        match *self {
            BinaryOperator::Equal => "=",
            BinaryOperator::BangEqual => "==",
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Star => "*",
            BinaryOperator::Slash => "/",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "<=",
        }
    }
}
