use crate::grover::{error, parser, token};
use error::Error;
use error::ErrorCode;
use parser::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use token::Token;
use token::Tokens;

pub struct Evaluator {
    variables: HashMap<String, f64>,
    constants: HashSet<String>,
    values: Tokens,
}

impl Evaluator {
    pub fn new() -> Self {
        let evaluator = Evaluator {
            variables: HashMap::<String, f64>::new(),
            constants: HashSet::<String>::new(),
            values: Tokens::new(),
        };

        evaluator
    }
    pub fn pop_value(&mut self) -> Result<f64, Error> {
        let token = if let Some(number) = self.values.pop() {
            number
        } else {
            return Err(Error{
                code: ErrorCode::EvaluatorError,
                message: format!("No tokens on stack."),
            });
        };
        match token {
            Token::Identifier(ref identifier) => {
                if let Some(number) = self.variables.get(identifier) {
                    Ok(*number)
                } else {
                    Err(Error{
                        code: ErrorCode::EvaluatorError,
                        message: format!("Undefined variable."),
                    })
                }
            }
            Token::Number(number) => {
                Ok(number)
            }
            _ => {
                Err(Error{
                    code: ErrorCode::EvaluatorError,
                    message: format!("Found non-value on value stack."),
                })
            }
        }
    }
    pub fn pop_ident(&mut self) -> Result<String, Error> {
        let token = if let Some(number) = self.values.pop() {
            number
        } else {
            return Err(Error{
                code: ErrorCode::EvaluatorError,
                message: format!("No tokens on stack."),
            });
        };
        match token {
            Token::Identifier(ref identifier) => {
                Ok(identifier.clone())
            }
            _ => {
                Err(Error{
                    code: ErrorCode::EvaluatorError,
                    message: format!("Found non-ident on value stack."),
                })
            }
        }
    }
    pub fn push_value(&mut self, value: Token) {
        self.values.push(value);
    }
    pub fn is_constant(&self, ident: &String) -> bool {
        self.constants.contains(ident)
    }
    pub fn evaluate(&mut self, tokens: Tokens) -> Result<f64, Error> {
        let ans = 0f64;

        for token in tokens.into_iter() {
            match token {
                Token::Number(_) => {
                    self.values.push(token);
                }
                Token::Identifier(ref identifier) => {
                    if !self.variables.contains_key(identifier) {
                        self.variables.insert(identifier.clone(), 0f64);
                    }
                    self.values.push(token);
                }
                Token::Operator(operator) => {
                    match *operator {
                        token::ADDITION_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            self.push_value(Token::Number(lhs + rhs));
                        }
                        token::SUBTRACTION_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            self.push_value(Token::Number(lhs - rhs));
                        }
                        token::MULTIPLICATION_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            self.push_value(Token::Number(lhs * rhs));
                        }
                        token::DIVISION_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            if rhs == 0f64 {
                                return Err(Error{
                                    code: ErrorCode::ArithmeticError,
                                    message: format!("Division by zero is undefined.")
                                });
                            }
                            self.push_value(Token::Number(lhs / rhs));
                        }
                        token::REMAINDER_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            if rhs == 0f64 {
                                return Err(Error{
                                    code: ErrorCode::ArithmeticError,
                                    message: format!("Division by zero is undefined.")
                                });
                            }
                            self.push_value(Token::Number(lhs % rhs));
                        }
                        token::POWER_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_value()?;
                            self.push_value(Token::Number(lhs.powf(rhs)));
                        }
                        token::ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() = rhs;
                            }
                        }
                        token::ADDITION_ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() = rhs;
                            }
                        }
                        token::SUBTRACTION_ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() -= rhs;
                            }
                        }
                        token::MULTIPLICATION_ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() *= rhs;
                            }
                        }
                        token::DIVISION_ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if rhs == 0f64 {
                                return Err(Error{
                                    code: ErrorCode::ArithmeticError,
                                    message: format!("Division by zero is undefined.")
                                });
                            }
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() /= rhs;
                            }
                        }
                        token::REMAINDER_ASSIGNMENT_OPERATOR => {
                            let rhs = self.pop_value()?;
                            let lhs = self.pop_ident()?;
                            if rhs == 0f64 {
                                return Err(Error{
                                    code: ErrorCode::ArithmeticError,
                                    message: format!("Division by zero is undefined.")
                                });
                            }
                            if self.is_constant(&lhs) {
                                return Err(Error{
                                    code: ErrorCode::ReassignConstant,
                                    message: format!("Can not reassign constant.")
                                });
                            }
                            if !self.variables.contains_key(&lhs) {
                                self.variables.insert(lhs, 0f64);
                            } else {
                                *self.variables.get_mut(&lhs).unwrap() %= rhs;
                            }
                        }
                        _ => {
                            return Err(Error{
                                code: ErrorCode::EvaluatorError,
                                message: format!("Unhandled operator."),
                            });
                        }
                    }
                }
                _ => {
                    return Err(Error{
                        code: ErrorCode::EvaluatorError,
                        message: format!("Unhandled token."),
                    });
                }
            };
        }

        if self.values.len() != 1 {
            return Err(Error{
                code: ErrorCode::EvaluatorError,
                message: format!("Evaluator does not have a stack size of one.")
            });
        }

        let result = self.pop_value()?;

        Ok(result)
    }
}
