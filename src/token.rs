use crate::{literal::Literal, token_type::TokenType};
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        lexeme: String,
        literal: Option<Literal>,
    ) -> Self {
        Token {
            token_type,
            line,
            lexeme,
            literal,
        }
    }
}
