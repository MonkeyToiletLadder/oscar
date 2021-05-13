pub enum ErrorCode {
    MalformedExpression,
    LexerError,
    ParserError,
    EvaluatorError,
    ArithmeticError,
    ReassignConstant,
}

pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}