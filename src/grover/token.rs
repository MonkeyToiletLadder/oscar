pub enum Associativity {
    Left,
    Right,
}

pub struct Operator {
    pub symbol: String,
    pub precedence: u8,
    pub associativity: Associativity,
}

pub enum Token {
    Identifier(String),
    Number(f64),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis,
}

pub struct TokenIterator<'a> {
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
    pub state: u8,
    pub error: String,
}