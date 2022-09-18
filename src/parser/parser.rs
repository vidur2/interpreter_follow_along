use crate::scanner::token::Token;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self {
            current: 0,
            tokens,
        }
    }
}