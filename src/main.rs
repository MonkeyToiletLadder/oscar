mod grover;

fn main() {
    let mut lexer = grover::TokenIterator::new("(5 + 4) * 2", 10);
    let mut parser = grover::Parser::new(lexer);
    let tokens = match parser.intermediate() {
        Ok(tokens) => tokens,
        Err(error) => {
            println!("{}", error.message);
            return;
        }
    };
    println!("{}", tokens);
    let mut eval = grover::Evaluator::new();
    let ans = eval.evaluate(tokens);
    println!("{:?}", ans)
    // println!("{}", token.get_error().unwrap());
}
