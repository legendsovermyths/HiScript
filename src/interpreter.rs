use crate::{
    error::{Error, ErrorManager, ErrorMessage},
    expr::Expr,
    literal::{
        Add, BitAnd, BitOr, Div, EqualTo, Greater, GreaterOrEqual, LeftShift, Lesser,
        LesserOrEqual, Literal, Mod, Mul, NotEqual, RightShift, Sub, Xor,
    },
    token::Token,
    token_type::TokenType,
};

pub struct Interpreter<'a> {
    error_manager: &'a mut ErrorManager,
}

impl<'a> Interpreter<'a> {
    pub fn new(error_manager: &'a mut ErrorManager) -> Self {
        Interpreter { error_manager }
    }

    pub fn interpret(&mut self, expr: Box<Expr>) -> Option<Literal> {
        let res = self.evaluate(expr);
        match res {
            Ok(val) => Some(val),
            Err(val) => {
                self.error_manager.add_runtime_error(val);
                None
            }
        }
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<Literal, Error> {
        match *expr {
            Expr::Literal(literal) => {
                return Ok(literal);
            }

            Expr::Grouping(expr) => {
                return self.evaluate(expr);
            }

            Expr::Unary(token, expr) => {
                let right = self.evaluate(expr)?;
                match token.token_type {
                    TokenType::BANG => Ok(Literal::Bool(!Self::is_true(right))),
                    TokenType::MINUS => match right {
                        Literal::Float(val) => Ok(Literal::Float(-val)),
                        Literal::Int(val) => Ok(Literal::Int(-val)),
                        _ => Err(Error::new(
                            token.line,
                            "Operand must be number".to_string(),
                            "".to_string(),
                        )),
                    },
                    _ => {
                        return Ok(Literal::None);
                    }
                }
            }
            Expr::Binary(expr_left, token, expr_right) => {
                let left = self.evaluate(expr_left)?;
                let right = self.evaluate(expr_right)?;
                match token.token_type {
                    TokenType::MINUS => Self::map_operator_result(left.sub(right), token),
                    TokenType::SLASH => Self::map_operator_result(left.div(right), token),
                    TokenType::STAR => Self::map_operator_result(left.mul(right), token),
                    TokenType::LESSLESS => Self::map_operator_result(left.left_shift(right), token),
                    TokenType::GREATERGREATER => {
                        Self::map_operator_result(left.right_shify(right), token)
                    }
                    TokenType::MODULO => Self::map_operator_result(left.modulo(right), token),
                    TokenType::AMPERSAND => Self::map_operator_result(left.bit_and(right), token),
                    TokenType::XOR => Self::map_operator_result(left.xor(right), token),
                    TokenType::PIPE => Self::map_operator_result(left.bit_or(right), token),
                    TokenType::PLUS => Self::map_operator_result(left.add(right), token),
                    TokenType::EQUALEQUAL => Self::map_operator_result(left.equal_to(right), token),
                    TokenType::GREATER => Self::map_operator_result(left.greater(right), token),
                    TokenType::GREATEREQUAL => {
                        Self::map_operator_result(left.greater_or_equal(right), token)
                    }
                    TokenType::LESS => Self::map_operator_result(left.lesser(right), token),
                    TokenType::LESSEQUAL => {
                        Self::map_operator_result(left.lesser_or_equal(right), token)
                    }
                    TokenType::BANGEQUAL => Self::map_operator_result(left.not_equal(right), token),
                    _ => {
                        return Ok(Literal::None);
                    }
                }
            }
            Expr::Ternary(left, mid, right) => {
                let left = self.evaluate(left)?;
                let mid = self.evaluate(mid)?;
                let right = self.evaluate(right)?;
                if Self::is_true(left) {
                    Ok(mid)
                } else {
                    Ok(right)
                }
            }
            _ => Ok(Literal::Bool(false)),
        }
    }
    fn is_true(literal: Literal) -> bool {
        match literal {
            Literal::None => false,
            Literal::Bool(val) => val,
            _ => true,
        }
    }

    fn map_operator_result(
        res: Result<Literal, ErrorMessage>,
        token: Token,
    ) -> Result<Literal, Error> {
        res.map_err(|err| Error::new(token.line, err.message, "".to_string()))
    }
}
