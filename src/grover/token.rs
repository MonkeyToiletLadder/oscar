pub enum Associativity {
    Left,
    Right,
}

pub struct Operator<'a> {
    pub symbol: &'a str,
    pub precedence: u8,
    pub associativity: Associativity,
}

pub enum Token<'a> {
    Identifier(&'a str),
    Number(f64),
    Operator(Operator<'a>),
    LeftParenthesis,
    RightParenthesis,
}

pub struct TokenIterator<'a> {
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
    pub state: u8,
    pub error: &'a str,
}

impl<'a> TokenIterator<'a> {
    const GOOD: u8 = 0b1u8;
    const BAD: u8 = 0b10u8;
    const END: u8 = 0b100u8;
    pub fn new(string: &'a str) -> TokenIterator<'a> {
        TokenIterator {
            chars: string.chars().peekable(),
            state: TokenIterator::GOOD,
            error: "",
        }
    }
    pub fn good(&self) -> bool {
        self.state & TokenIterator::GOOD != 0
    }
    pub fn bad(&self) -> bool {
        self.state & TokenIterator::BAD != 0
    }
    pub fn end(&self) -> bool {
        self.state & TokenIterator::END != 0
    }
}