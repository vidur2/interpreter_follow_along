use crate::{ast::expr_types::Scope, scanner::token::Token};

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
    InvalidEnv(Scope),
    InvalidEnvAssign(Token),
    InvalidEnvCall(Token),
    InvalidLoop(Token),
    InvalidIndex(Token),
    InvalidAppend(Token),
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
            Self::InvalidAppend(tok) => format!("Invalid call of append on line {}", tok.line),
            Self::InvalidIndex(tok) => format!("Parsing Error: Invalid Index on line {}", tok.line),
            ParsingException::PlaceHolder => {
                String::from("Parsing Error: Limitation of rust borrow checker")
            }
            ParsingException::InvalidPrint(tok) => format!(
                "Parsing Error: Invalid print statement on line {}",
                tok.line
            ),
            Self::InvalidAssign(tok) => format!(
                "Parsing Error: missing '=' after ident on line {}",
                tok.line
            ),
            Self::InvalidIdentifier(tok) => {
                format!("Parsing Error: Invalid variable expr on line {}", tok.line)
            }
            Self::InvalidEnv(env) => format!(
                "Parsing Error: non-variable declaration in environment '{}' on line {}",
                env.ident.clone().unwrap().lexeme,
                env.ident.clone().unwrap().line
            ),
            Self::InvalidEnvAssign(tok) => format!(
                "Invalid environment assignment of '{}' on line {}",
                tok.lexeme, tok.line
            ),
            Self::InvalidEnvCall(tok) => format!("Invalid environment call on line {}", tok.line),
            ParsingException::InvalidLoop(tok) => {
                format!("Invalid {:?} loop on line {}", tok.tok, tok.line)
            }
        }
    }
}
