use crate::scanner::token::Token;

use super::error_reporter::Unwindable;

pub type Result<'a, T> = std::result::Result<T, ParsingException>;

#[derive(Clone, Debug)]
pub enum ParsingException {
    UnterminatedParenthesis(Token),
    InvalidExpr(Token),
    PlaceHolder,
}

impl Unwindable for ParsingException {
    fn get_value(&self) -> String {
        match self {
            Self::UnterminatedParenthesis(tok) => format!(
                "Parsing Errror: Unterminated Parenthesis on line: {}",
                tok.line
            ),
            Self::InvalidExpr(tok) => format!("Parsing Error: Invalid Expression on line: {}", tok.line),
            ParsingException::PlaceHolder => String::from("Limitation of rust borrow checker"),
        }
    }
}
