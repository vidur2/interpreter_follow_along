use crate::{
    error_reporting::{interp_err::InterpException, parsing_err::ParsingException},
    scanner::token::Primitive,
};

use super::expr_types::ExprPossibilities;

pub trait Accept {
    fn accept<P: Interperable<Result<Primitive, InterpException>>>(
        expr: ExprPossibilities,
        visitor: &mut P,
    ) -> Result<Primitive, InterpException> {
        return visitor.visit_expr(expr);
    }
}

pub trait Interperable<R> {
    fn visit_expr(&mut self, expr: ExprPossibilities) -> R;
}
