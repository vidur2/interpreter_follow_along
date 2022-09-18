use std::pin::Pin;

use crate::scanner::token::{Primitive, Token};

use super::ast_traits::Accept;

#[derive(Clone)]
pub struct Expr {
    pub left: Box<Pin<Option<Expr>>>,
    pub right: Box<Pin<Option<Expr>>>,
    pub operator: Token,
}

pub struct Binary {
    pub left: Expr,
    pub right: Expr,
    pub operator: Token,
}

pub struct Grouping {
    pub expr: Expr,
}

pub struct Literal {
    pub literal: Primitive,
}

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

pub enum ExprPossibilities {
    Expr(Expr),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Accept<Option<String>> for ExprPossibilities {}
