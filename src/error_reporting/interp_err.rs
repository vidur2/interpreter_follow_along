use crate::{scanner::token::{TokenType}, ast::expr_types::{Unary, Binary}};

use super::error_reporter::Unwindable;

pub type Result<'a, T> = std::result::Result<T, InterpException>;

#[derive(Clone, Debug)]
pub enum InterpException {
    InvalidUnary(Unary),
    InvalidBinary(Binary),
    DivideByZero(Binary),
    InvalidTernaryExpr(u64),
    IdentifierNoExist(String),
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
            InterpException::InvalidTernaryExpr(line) => format!("Used string in ternary expr on line {}", line),
            InterpException::InvalidBinary(binary) => format!("Invalid Expression on line {}", binary.operator.line), 
            InterpException::PlaceHolder => String::from("Limitation of rust borrow checker"),
            InterpException::IdentifierNoExist(ident) => format!("Identifier '{}' does not exist", ident),
            InterpException::DivideByZero(_) => todo!(),
        }
    }
}