use crate::grover::{
    parser,
    token,
    error,
};
use parser::Parser;
use token::Token;
use error::Error;
use error::ErrorCode;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Evaluator {
    variables: HashMap<String, f64>,
    constants: HashSet<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        let evaluator = Evaluator {
            variables: HashMap::<String, f64>::new(),
            constants: HashSet::<String>::new(),
        };
        
        evaluator
    }
    pub fn evaluate<'a>(&mut self, tokens: Vec<Token<'a>>) -> Result<f64, Error> {
        let ans = 0f64;
        
        let numbers = Vec::<f64>::new();

        for token in tokens.iter() {

        }

        Ok(ans)
    }
}