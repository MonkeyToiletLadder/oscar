#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Operator<'a> {
    pub symbol: &'a str,
    pub precedence: u8,
    pub associativity: Associativity,
}

// *********
// Operators
// *********

pub const ADDITION_OPERATOR: Operator = Operator {
    symbol: "+",
    precedence: 2,
    associativity: Associativity::Left,
};

pub const SUBTRACTION_OPERATOR: Operator = Operator {
    symbol: "-",
    precedence: 2,
    associativity: Associativity::Left,
};

pub const MULTIPLICATION_OPERATOR: Operator = Operator {
    symbol: "*",
    precedence: 1,
    associativity: Associativity::Left,
};

pub const DIVISION_OPERATOR: Operator = Operator {
    symbol: "/",
    precedence: 1,
    associativity: Associativity::Left,
};

pub const POWER_OPERATOR: Operator = Operator {
    symbol: "^",
    precedence: 0,
    associativity: Associativity::Right,
};

pub const REMAINDER_OPERATOR: Operator = Operator {
    symbol: "%",
    precedence: 1,
    associativity: Associativity::Left,
};

pub const ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "=",
    precedence: 3,
    associativity: Associativity::Right,
};

pub const ADDITION_ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "+=",
    precedence: 3,
    associativity: Associativity::Right,
};

pub const SUBTRACTION_ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "-=",
    precedence: 3,
    associativity: Associativity::Right,
};

pub const MULTIPLICATION_ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "*=",
    precedence: 3,
    associativity: Associativity::Right,
};

pub const DIVISION_ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "/=",
    precedence: 3,
    associativity: Associativity::Right,
};

pub const REMAINDER_ASSIGNMENT_OPERATOR: Operator = Operator {
    symbol: "%=",
    precedence: 3,
    associativity: Associativity::Right,
};

// ******
// Tokens
// ******

pub const ADDITION_TOKEN: Token = Token::Operator(ADDITION_OPERATOR);

pub const SUBTRACTION_TOKEN: Token = Token::Operator(SUBTRACTION_OPERATOR);

pub const MULTIPLICATION_TOKEN: Token = Token::Operator(MULTIPLICATION_OPERATOR);

pub const DIVISION_TOKEN: Token = Token::Operator(DIVISION_OPERATOR);

pub const POWER_TOKEN: Token = Token::Operator(POWER_OPERATOR);

pub const REMAINDER_TOKEN: Token = Token::Operator(REMAINDER_OPERATOR);

pub const ASSIGNMENT_TOKEN: Token = Token::Operator(ASSIGNMENT_OPERATOR);

pub const ADDITION_ASSIGNMENT_TOKEN: Token = Token::Operator(ADDITION_ASSIGNMENT_OPERATOR);

pub const SUBTRACTION_ASSIGNMENT_TOKEN: Token = Token::Operator(SUBTRACTION_ASSIGNMENT_OPERATOR);

pub const MULTIPLICATION_ASSIGNMENT_TOKEN: Token = Token::Operator(MULTIPLICATION_ASSIGNMENT_OPERATOR);

pub const DIVISION_ASSIGNMENT_TOKEN: Token = Token::Operator(DIVISION_ASSIGNMENT_OPERATOR);

pub const REMAINDER_ASSIGNMENT_TOKEN: Token = Token::Operator(REMAINDER_ASSIGNMENT_OPERATOR);
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(String),
    Number(f64),
    Operator(Operator<'a>),
    LeftParenthesis,
    RightParenthesis,
}

pub struct TokenIterator<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    state: u8,
    error: String,
    radix: u32,
}

impl<'a> TokenIterator<'a> {
    const GOOD: u8 = 0b1u8;
    const BAD: u8 = 0b10u8;
    const END: u8 = 0b100u8;
    pub fn new(string: &'a str, radix: u32) -> TokenIterator<'a> {
        TokenIterator {
            chars: string.chars().peekable(),
            state: TokenIterator::GOOD,
            error: "".to_string(),
            radix
        }
    }
    pub fn state(&mut self, mask: u8) {
        self.state |= mask;
    }
    pub fn clear(&mut self, mask: u8) {
        self.state &= !mask;
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
    pub fn get_error(&self) -> Option<&String> {
        if self.bad() {
            return Some(&self.error);
        }
        None
    }
}   

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<<TokenIterator<'a> as Iterator>::Item> {
        let character: char = match self.chars.next() {
            Some(character) => character,
            None => {
                self.state(TokenIterator::END);
                return None;
            }
        };
        match character {
            // Spaces
            ' ' => return self.next(),
            // Variables
            '$' => {
                let mut identifier = String::from("$");
                if let Some(character) = self.chars.peek() {
                    if !(character.is_alphabetic() || *character == '_') {
                        self.state &= !TokenIterator::GOOD;
                        self.state |= TokenIterator::BAD;
                        self.error = format!("Variable name must start with a letter or undercore. Found \'{}\'.", character);
                        return None;
                    } else {
                        identifier += &character.to_string();
                        self.chars.next();
                    }
                } else {
                    self.clear(TokenIterator::GOOD);
                    self.state(TokenIterator::BAD);
                    self.error = format!("Variable name must be at least on character long.");
                    return None;
                }
                loop {
                    if let Some(character) = self.chars.peek() {
                        if character.is_alphanumeric() || *character == '_' {
                            identifier += &character.to_string();
                            self.chars.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                return Some(Token::Identifier(identifier));
            }
            '(' => return Some(Token::LeftParenthesis),
            ')' => return Some(Token::RightParenthesis),
            // Assignment
            '=' => return Some(ASSIGNMENT_TOKEN),
            // Operators
            _ if character == '+'
                || character == '-'
                || character == '*'
                || character == '/'
                || character == '%'
                || character == '^' =>
            {
                // Assignment Operators
                if self.chars.peek() == Some(&'=') {
                    return match character {
                        '+' => {
                            self.chars.next();
                            Some(ADDITION_ASSIGNMENT_TOKEN)
                        }
                        '-' => {
                            self.chars.next();
                            Some(SUBTRACTION_ASSIGNMENT_TOKEN)
                        }
                        '*' => {
                            self.chars.next();
                            Some(MULTIPLICATION_ASSIGNMENT_TOKEN)
                        }
                        '/' => {
                            self.chars.next();
                            Some(DIVISION_ASSIGNMENT_TOKEN)
                        }
                        '%' => {
                            self.chars.next();
                            Some(REMAINDER_ASSIGNMENT_TOKEN)
                        }
                        // Unhandled Assignment Operator
                        _ => {
                            self.clear(TokenIterator::GOOD);
                            self.state(TokenIterator::BAD);
                            self.error = format!("Unhandled character \'{}\'", character);
                            None
                        }
                    };
                } else {
                    // Arithmetic Operators
                    return match character {
                        '+' => Some(ADDITION_TOKEN),
                        '-' => Some(SUBTRACTION_TOKEN),
                        '*' => Some(MULTIPLICATION_TOKEN),
                        '/' => Some(DIVISION_TOKEN),
                        '%' => Some(REMAINDER_TOKEN),
                        '^' => Some(POWER_TOKEN),
                        // Unhandled Operator
                        _ => {
                            self.clear(TokenIterator::GOOD);
                            self.state(TokenIterator::BAD);
                            self.error = format!("Unhandled character \'{}\'", character);
                            None
                        }
                    };
                }
            }
            // Numbers
            _ if character.is_digit(self.radix) || if self.radix == 10 { true } else { false } && character == '.' => {
                let mut number = String::from(&character.to_string());
                loop {
                    if let Some(&character) = self.chars.peek() {
                        if character.is_digit(self.radix) || if self.radix == 10 { true } else { false } && character == '.' {
                            number += &character.to_string();
                            self.chars.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if self.radix == 10 {
                    let number: f64 = match number.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            self.clear(TokenIterator::GOOD);
                            self.state(TokenIterator::BAD);
                            self.error = format!("Could not parse \'{}\' to f64.", number);
                            return None;
                        }
                    };
                    return Some(Token::Number(number));
                } else {
                    let number = match i64::from_str_radix(&number, self.radix) {
                        Ok(number) => number as f64,
                        Err(_) => {
                            self.clear(TokenIterator::GOOD);
                            self.state(TokenIterator::BAD);
                            self.error = format!("Could not parse \'{}\' to f64.", number);
                            return None;
                        }
                    };
                    return Some(Token::Number(number));
                }
            }
            // Invalid Characters
            _ => {
                self.clear(TokenIterator::GOOD);
                self.state(TokenIterator::BAD);
                self.error = format!("Invalid character \'{}\'", character);
                return None;
            }
        };
        // No Characters Found
        self.state(TokenIterator::END);
        None
    }
}

pub struct Tokens<'a> {
    raw: Vec<Token<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn new() -> Self {
        Tokens {
            raw: Vec::<Token<'a>>::new(),
        }
    }
    pub fn push(&mut self, token: Token<'a>) {
        self.raw.push(token);
    }
}

impl<'a> std::fmt::Display for Tokens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");

        for token in &self.raw {
            match token {
                Token::Identifier(identifier) => output += &identifier,
                Token::Number(number) => output += &number.to_string(),
                Token::LeftParenthesis => output += "(",
                Token::RightParenthesis => output += ")",
                Token::Operator(operator) => output += operator.symbol,
            }
            output += " ";
        }

        write!(f, "{}", output)
    }
}