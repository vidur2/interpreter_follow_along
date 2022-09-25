use crate::{scanner::token::{Token, TokenType}, ast::ast_types::{Unary, Binary}};

use super::error_reporter::Unwindable;

pub type Result<'a, T> = std::result::Result<T, InterpException>;

#[derive(Clone, Debug)]
pub enum InterpException {
    InvalidUnary(Unary),
    InvalidBinary(Binary),
    PlaceHolder,
}

impl InterpException {
    fn get_other_unary(tok_type: TokenType) -> char {
        if let TokenType::MINUS = tok_type  {
            return '!'
        } else {
            return '-'
        }
    }
}

impl Unwindable for InterpException {
    fn get_value(&self) -> String {
        match self {
            InterpException::InvalidUnary(tok) => format!("Invalid unary expr on line {}", tok.operator.line),
            InterpException::InvalidBinary(_) => todo!(),
            InterpException::PlaceHolder => String::from("Limitation of rust borrow checker"),
        }
    }
}