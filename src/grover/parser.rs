use crate::grover::{
    token,
    error
};
use token::Token;
use token::TokenIterator;
use error::Error;
use error::ErrorCode;

pub struct Parser<'a> {
    stream: TokenIterator<'a>
}

impl<'a> Parser<'a> {
    pub fn new(stream: TokenIterator<'a>) -> Self {
        Parser {
            stream,
        }
    }
    pub fn intermediate(&mut self) -> Result<Vec<Token<'a>>, Error> {
        let tokens = Vec::<Token<'a>>::new();
        return Ok(tokens);
    }
}