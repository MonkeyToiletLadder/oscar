use crate::grover::{error, token};
use error::Error;
use error::ErrorCode;
use token::Token;
use token::Tokens;
use token::TokenIterator;
use token::Associativity;

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

        while let Some(token) = self.stream.next() {
            match token {
                Token::Identifier(_) | Token::Number(_) => {
                    tokens.push(token);
                }
                Token::Operator(ref operator) => {
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
                    operators.push(token);
                }
                Token::RightParenthesis => {
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
