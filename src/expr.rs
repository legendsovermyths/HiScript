use crate::{literal::Literal, token::Token};
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl Expr {
    fn printer(self) {
        
    }
}
