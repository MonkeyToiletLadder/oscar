pub enum ErrorCode {
    MalformedExpression,
    LexerError,
    ParserError,
    EvaluatorError,
    ArithmeticError,
}

pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}