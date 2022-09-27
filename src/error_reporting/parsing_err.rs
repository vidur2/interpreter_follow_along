use crate::{scanner::token::Token, ast::expr_types::EnvParsable};

use super::error_reporter::Unwindable;

pub type Result<'a, T> = std::result::Result<T, ParsingException>;

#[derive(Clone, Debug)]
pub enum ParsingException {
    UnterminatedParenthesis(Token),
    InvalidExpr(Token),
    InvalidTernaryExpr(Token),
    InvalidPrint(Token),
    InvalidAssign(Token),
    InvalidIdentifier(Token),
    InvalidEnv(EnvParsable),
    InvalidEnvAssign(Token),
    PlaceHolder,
}

impl Unwindable for ParsingException {
    fn get_value(&self) -> String {
        match self {
            Self::UnterminatedParenthesis(tok) => format!(
                "Parsing Errror: Unterminated Parenthesis on line: {}",
                tok.line
            ),
            Self::InvalidExpr(tok) => {
                format!("Parsing Error: Invalid Expression on line: {}", tok.line)
            }
            Self::InvalidTernaryExpr(tok) => format!(
                "Parsing Error: Invalid Ternary Expression on line: {}",
                tok.line - 1
            ),
            ParsingException::PlaceHolder => String::from("Limitation of rust borrow checker"),
            ParsingException::InvalidPrint(tok) => format!("Parsing Error: Invalid print statement on line {}", tok.line),
            Self::InvalidAssign(tok) => format!("Parsing Error: missing '=' after ident on line {}", tok.line),
            Self::InvalidIdentifier(tok) => format!("Parsing Error: Invalid variable expr on line {}", tok.line),
            Self::InvalidEnv(env) => format!("Parsing Error: non-variable declaration in environment '{}' on line {}", env.ident.lexeme, env.ident.line),
            Self::InvalidEnvAssign(tok) => format!("Invalid environment assignemnt of '{}' on line {}", tok.lexeme, tok.line)
        }
    }
}
