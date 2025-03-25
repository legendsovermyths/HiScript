use std::{
    collections::HashMap,
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::{error::ErrorManager, literal::Literal, token::Token, token_type::TokenType};

pub struct Lexer<'a> {
    source: String,
    iter: Peekable<Enumerate<Chars<'a>>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenType>,
    error_manager: &'a mut ErrorManager,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String, error_manager: &'a mut ErrorManager) -> Self {
        let iter = source.chars().enumerate().peekable();
        let source = source.to_string();
        let mut keywords: HashMap<&str, TokenType> = HashMap::new();
        keywords.insert("and", TokenType::AND);
        keywords.insert("or", TokenType::OR);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("if", TokenType::IF);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);
        Lexer {
            source,
            iter,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
            error_manager,
        }
    }

    pub fn scan_tokens(&mut self) {
        while let Some((index, _)) = self.iter.peek() {
            self.start = *index;
            self.scan_token();
        }
        self.add_token(TokenType::EOF, None);
    }

    fn does_match(&mut self, c: char) -> bool {
        if let Some((index, curr_char)) = self.iter.peek() {
            if *curr_char == c {
                self.current = *index;
                self.iter.next();
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn match_string(&mut self) {
        while let Some((index, curr_char)) = self.iter.peek() {
            if *curr_char == '"' {
                let string_literal = self.source[(self.current + 1)..(*index)].to_string();
                self.add_token(TokenType::STRING, Some(Literal::String(string_literal)));
                self.iter.next();
                return;
            } else if *curr_char == '\n' {
                self.line += 1;
            }
            self.iter.next();
        }
        (*self.error_manager).add_error(
            self.line,
            "Unterminated String".to_string(),
            "".to_string(),
        );
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        let mut has_dot = false;
        let mut last_index = self.current;

        while let Some((index, curr_char)) = self.iter.peek() {
            if Self::is_digit(*curr_char) {
                last_index = *index;
                self.iter.next();
            } else if *curr_char == '.' {
                if has_dot {
                    break;
                }
                has_dot = true;
                self.iter.next();
            } else {
                break;
            }
        }

        if has_dot {
            if last_index == self.current {
                let integer: i64 = self.source[self.current..last_index].parse().unwrap();
                self.add_token(TokenType::INTEGER, Some(Literal::Int(integer)));
                self.add_token(TokenType::DOT, None);
            } else {
                let decimal: f64 = self.source[self.current..=last_index].parse().unwrap();
                self.add_token(TokenType::FLOAT, Some(Literal::Float(decimal)));
            }
        } else {
            let integer: i64 = self.source[self.current..=last_index].parse().unwrap();
            self.add_token(TokenType::INTEGER, Some(Literal::Int(integer)));
        }
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    fn identifier(&mut self) {
        while let Some((index, curr_char)) = self.iter.peek() {
            if Self::is_alphanumeric(*curr_char) {
                self.iter.next();
            } else {
                let word = &self.source[self.current..*index];
                if let Some(keyword_type) = self.keywords.get(word) {
                    self.add_token(*keyword_type, None);
                    return;
                }
                self.add_token(TokenType::IDENTIFIER, None);
                return;
            }
        }
    }

    fn scan_token(&mut self) {
        if let Some((index, c)) = self.iter.next() {
            self.current = index;
            match c {
                '(' => self.add_token(TokenType::LEFTPAREN, None),
                ')' => self.add_token(TokenType::RIGHTPAREN, None),
                '{' => self.add_token(TokenType::LEFTBRACE, None),
                '}' => self.add_token(TokenType::RIGHTBRACE, None),
                '.' => self.add_token(TokenType::DOT, None),
                ',' => self.add_token(TokenType::COMMA, None),
                '+' => self.add_token(TokenType::PLUS, None),
                '-' => self.add_token(TokenType::MINUS, None),
                '*' => self.add_token(TokenType::STAR, None),
                ';' => self.add_token(TokenType::SEMICOLON, None),
                '^' => self.add_token(TokenType::XOR, None),
                '%' => self.add_token(TokenType::MODULO, None),
                '&' => self.add_token(TokenType::AMPERSAND, None),
                '|' => self.add_token(TokenType::PIPE, None),
                '!' => {
                    if self.does_match('=') {
                        self.add_token(TokenType::BANGEQUAL, None)
                    } else {
                        self.add_token(TokenType::BANG, None)
                    }
                }
                '=' => {
                    if self.does_match('=') {
                        self.add_token(TokenType::EQUALEQUAL, None)
                    } else {
                        self.add_token(TokenType::EQUAL, None)
                    }
                }
                '>' => {
                    if self.does_match('=') {
                        self.add_token(TokenType::GREATEREQUAL, None)
                    } else if self.does_match('>') {
                        self.add_token(TokenType::GREATERGREATER, None)
                    } else {
                        self.add_token(TokenType::GREATER, None)
                    }
                }
                '<' => {
                    if self.does_match('=') {
                        self.add_token(TokenType::LESSEQUAL, None)
                    } else if self.does_match('<') {
                        self.add_token(TokenType::LESSLESS, None)
                    } else {
                        self.add_token(TokenType::LESS, None)
                    }
                }
                '/' => {
                    if self.does_match('/') {
                        while let Some((_index, curr_char)) = self.iter.peek() {
                            if *curr_char == '\n' {
                                self.line += 1;
                                break;
                            } else {
                                self.iter.next();
                            }
                        }
                    } else {
                        self.add_token(TokenType::SLASH, None);
                    }
                }
                ' ' => (),
                '\t' => (),
                '\r' => (),
                '\n' => self.line += 1,
                '"' => self.match_string(),
                _ => {
                    if Self::is_digit(c) {
                        self.number();
                    } else if Self::is_alpha(c) {
                        self.identifier();
                    } else {
                        self.error_manager.add_error(
                            self.line,
                            "Unexpected token".to_string(),
                            "".to_string(),
                        );
                    }
                }
            }
        }
    }
    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, self.line, lexeme, literal));
    }
    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            print!("{:?}", token);
        }
    }
}
