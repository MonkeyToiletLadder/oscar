use crate::grover::{error, token};
use error::Error;
use error::ErrorCode;
use token::Token;
use token::Tokens;
use token::TokenIterator;
use token::Associativity;
use std::collections::HashSet;

pub struct Parser<'a> {
    stream: TokenIterator<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(stream: TokenIterator<'a>) -> Self {
        Parser { stream }
    }
    pub fn intermediate(&mut self) -> Result<Tokens<'a>, Error> {
        let mut tokens = Tokens::<'a>::new();

        // Shunting Yard Algorithm

        let mut operators = Vec::<Token<'a>>::new();

        let mut parenthesis_depth = 0;

        let mut expected = HashSet::<&str>::new();

        expected.insert("identifier");
        expected.insert("number");
        expected.insert("unary-plus");
        expected.insert("unary-minus");
        expected.insert("left-parenthesis");

        while let Some(token) = self.stream.next() {
            match token {
                Token::Identifier(_) => {
                    if !(expected.contains("identifier")) {
                        return Err(Error{
                            code: ErrorCode::MalformedExpression,
                            message: format!("Expected {:?} found identifier.", expected),
                        });
                    }
                    expected.clear();
                    expected.insert("assignment-operator");
                    expected.insert("arithmetic-operator");
                    expected.insert("right-parenthesis");
                    tokens.push(token);
                }
                Token::Number(_) => {
                    if !(expected.contains("number")) {
                        return Err(Error{
                            code: ErrorCode::MalformedExpression,
                            message: format!("Expected {:?} found number.", expected),
                        });
                    }
                    expected.clear();
                    expected.insert("assignment-operator");
                    expected.insert("arithmetic-operator");
                    expected.insert("right-parenthesis");
                    tokens.push(token);
                }
                Token::Operator(ref operator) => {
                    match *operator {
                        token::ADDITION_OPERATOR => {
                            if !expected.contains("unary-plus") && !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found unary-plus.", expected),
                                });
                            } 
                            if !expected.contains("arithmetic-operator") && !expected.contains("unary-plus") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found plus-operator.", expected),
                                });
                            } 
                            if expected.contains("unary-plus") {
                                expected.clear();
                                expected.insert("unary-plus");
                                expected.insert("unary-minus");
                                expected.insert("identifier");
                                expected.insert("number");
                                tokens.push(Token::Number(1f64));
                                operators.push(token::MULTIPLICATION_TOKEN);
                                continue;
                            } else {
                                expected.clear();
                                expected.insert("unary-plus");
                                expected.insert("unary-minus");
                                expected.insert("identifier");
                                expected.insert("number");
                                expected.insert("left-parenthesis");
                            }
                        }
                        token::SUBTRACTION_OPERATOR => {
                            if !expected.contains("unary-minus") && !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found unary-minus.", expected),
                                });
                            } 
                            if !expected.contains("arithmetic-operator") && !expected.contains("unary-minus") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found minus-operator.", expected),
                                });
                            } 
                            if expected.contains("unary-minus") {
                                expected.clear();
                                expected.insert("unary-plus");
                                expected.insert("unary-minus");
                                expected.insert("identifier");
                                expected.insert("number");
                                expected.insert("left-parenthesis");
                                tokens.push(Token::Number(-1f64));
                                operators.push(token::MULTIPLICATION_TOKEN);
                                continue;
                            } else {
                                expected.clear();
                                expected.insert("unary-plus");
                                expected.insert("unary-minus");
                                expected.insert("identifier");
                                expected.insert("number");
                                expected.insert("left-parenthesis");
                            }
                        },
                        token::MULTIPLICATION_OPERATOR => {
                            if !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found times-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::DIVISION_OPERATOR => {
                            if !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found division-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::REMAINDER_OPERATOR => {
                            if !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found remainder-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::POWER_OPERATOR => {
                            if !expected.contains("arithmetic-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found power-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::ADDITION_ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found addition-assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::SUBTRACTION_ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found subtraction-assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::MULTIPLICATION_ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found multiplication-assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::DIVISION_ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found division-assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        token::REMAINDER_ASSIGNMENT_OPERATOR => {
                            if !expected.contains("assignment-operator") {
                                return Err(Error{
                                    code: ErrorCode::MalformedExpression,
                                    message: format!("Expected {:?} found remainder-assignment-operator.", expected),
                                });
                            }
                            expected.clear();
                            expected.insert("unary-minus");
                            expected.insert("unary-plus");
                            expected.insert("number");
                            expected.insert("identifier");
                            expected.insert("left-parenthesis");
                        },
                        _ => {
                            return Err(Error{
                                code: ErrorCode::ParserError,
                                message: format!("Unhandled operator {:?}", operator),
                            });
                        }
                    };
                    loop {
                        if let Some(&ref last_token) = operators.last() {
                            match last_token {
                                Token::Operator(ref last_operator) => {
                                    if last_operator.precedence < operator.precedence
                                        || last_operator.precedence == operator.precedence
                                            && operator.associativity == Associativity::Left
                                    {
                                        tokens.push(operators.pop().expect("Operator stack has valid last value but pop failed to retrieve it."));
                                    } else {
                                        break;
                                    }
                                }
                                // These Operators are Handled Elsewhere
                                Token::LeftParenthesis => {
                                    break;
                                },
                                Token::RightParenthesis => {
                                    tokens.push(operators.pop().expect("Operator stack has valid last value but pop failed to retrieve it."));
                                },
                                _ => {
                                    return Err(Error{
                                        code: ErrorCode::ParserError,
                                        message: format!("Encountered a non-operator in operator stack."),
                                    });
                                }
                            };
                        } else {
                            break;
                        }
                    }
                    operators.push(token);
                }
                Token::LeftParenthesis => {
                    if !(expected.contains("left-parenthesis")) {
                        return Err(Error{
                            code: ErrorCode::MalformedExpression,
                            message: format!("Expected {:?} found left-parenthesis.", expected),
                        });
                    }
                    parenthesis_depth += 1;
                    operators.push(token);
                }
                Token::RightParenthesis => {
                    if parenthesis_depth == 0 {
                        return Err(Error{
                            code: ErrorCode::MalformedExpression,
                            message: format!("Dangling right parenthesis."),
                        });
                    }
                    loop {
                        if let Some(&ref last_token) = operators.last() {
                            if *last_token != Token::LeftParenthesis {
                                tokens.push(operators.pop().expect("Operator stack has valid last value but pop failed to retrieve it."));
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }                 
                    }
                    if let Some(&ref last_token) = operators.last() {
                        if *last_token == Token::LeftParenthesis {
                            operators.pop();
                        }
                    }
                }
            };
        }

        while operators.len() > 0 {
            tokens.push(operators.pop().expect("Operator stack has valid last value but pop failed to retrieve it."));
        }

        if let Some(error) = self.stream.get_error() {
            return Err(Error{
                code: ErrorCode::LexerError,
                message: format!("{}", error),
            });
        } else {
            return Ok(tokens);
        }
    }
}
