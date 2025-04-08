use std::error::Error;

use crate::{
    error::ErrorManager, expr::Expr, literal::Literal, token::Token, token_type::TokenType,
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    error_manager: &'a mut ErrorManager,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_manager: &'a mut ErrorManager) -> Self {
        Parser {
            current: 0,
            tokens,
            error_manager,
        }
    }

    pub fn parse(&mut self) -> Option<Box<Expr>> {
        let result = self.expression();
        result.ok()
    }

    fn expression(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let result = self.block()?;
        return Ok(result);
    }

    fn block(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.ternary()?;
        while self.does_match(vec![TokenType::COMMA]) {
            let operator = self.previous().clone();
            let right = self.ternary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.equality()?;
        if self.does_match(vec![TokenType::QUESTION]) {
            let expr_then = self.ternary()?;
            self.consume(TokenType::COLON, "Expected ':' in ternay operation");
            let expr_else = self.ternary()?;
            expr = Box::new(Expr::Ternary(expr, expr_then, expr_else));
        }
        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.comparision()?;
        while self.does_match(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparision()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn comparision(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.logic()?;
        while self.does_match(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.logic()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn logic(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.shift()?;
        while self.does_match(vec![TokenType::AMPERSAND, TokenType::PIPE, TokenType::XOR]) {
            let operator = self.previous().clone();
            let right = self.shift()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn shift(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.term()?;
        while self.does_match(vec![TokenType::LESSLESS, TokenType::GREATERGREATER]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.factor()?;
        while self.does_match(vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.modulo()?;
        while self.does_match(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.modulo()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn modulo(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        let mut expr = self.unary()?;
        while self.does_match(vec![TokenType::MODULO]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        if self.does_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<Expr>, Box<dyn Error>> {
        if self.does_match(vec![TokenType::TRUE]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(true))));
        } else if self.does_match(vec![TokenType::FALSE]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(false))));
        } else if self.does_match(vec![TokenType::NIL]) {
            return Ok(Box::new(Expr::Literal(Literal::None)));
        } else if self.does_match(vec![
            TokenType::STRING,
            TokenType::INTEGER,
            TokenType::FLOAT,
        ]) {
            return Ok(Box::new(Expr::Literal(
                self.previous().literal.clone().unwrap(),
            )));
        } else if self.does_match(vec![TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHTPAREN, "Expected ')' after expression");
            return Ok(Box::new(Expr::Grouping(expr)));
        }
        self.report_error("Expected an expression");
        return Err("Expected an expression".into());
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
            return;
        }
        self.report_error(message);
    }

    fn report_error(&mut self, message: &str) {
        let token = self.previous();
        if token.token_type == TokenType::EOF {
            self.error_manager
                .add_error(token.line, message.to_string(), "at end".to_string());
        } else {
            let why = format!("at '{}'", token.lexeme).to_string();
            self.error_manager
                .add_error(token.line, message.to_string(), why);
        }
    }
    fn synchronize(&mut self) {
        self.advance();
        while (!self.is_at_end()) {
            if (self.previous().token_type == TokenType::SEMICOLON) {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS => return,
                TokenType::IF => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => (),
            }
            self.advance();
        }
    }
    fn does_match(&mut self, token_list: Vec<TokenType>) -> bool {
        for token in token_list.iter() {
            if self.check(*token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }

    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }
}
