mod grover;

fn main() {
    let mut token = grover::TokenIterator::new("$age = 5", 10);
    while token.good() && !token.end() {
        println!("{:?}", token.next());
        if let Some(error) = token.get_error() {
            println!("{}", error);
            break;
        } else {
            
        }
    }
    // println!("{}", token.get_error().unwrap());
}
