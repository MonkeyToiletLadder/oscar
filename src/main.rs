mod grover;

fn main() {
    let mut lexer = grover::TokenIterator::new("(1 + (1 * 1))^(1 + 1)", 10);
    let mut parser = grover::Parser::new(lexer);
    let tokens = match parser.intermediate() {
        Ok(tokens) => tokens,
        Err(error) => {
            println!("{}", error.message);
            return;
        }
    };
    println!("{}", tokens);
    // println!("{}", token.get_error().unwrap());
}
