pub enum ErrorCode {
    MalformedExpression,
    LexerError,
    ParserError,
    EvaluatorError,
    ArithmeticError,
}

pub struct Error<'a> {
    pub code: ErrorCode,
    pub message: &'a str,
}