use crate::{ast::{ast_traits::{Interperable, Accept}, ast_types::{ExprPossibilities, Expr}}, scanner::token::{Primitive, TokenType}, error_reporting::interp_err::InterpException};

pub struct Interpreter {

}


impl Interpreter {
    fn evaluate(&self, expr: &ExprPossibilities) -> Result<Primitive, InterpException> {
        return ExprPossibilities::accept(expr.clone(), self);
    }
}

impl Interperable<Result<Primitive, InterpException>> for &Interpreter {
    fn visit_expr(&self, expr: crate::ast::ast_types::ExprPossibilities) -> Result<Primitive, InterpException> {
        match expr {
            crate::ast::ast_types::ExprPossibilities::Expr(_) => todo!(),
            crate::ast::ast_types::ExprPossibilities::Binary(_) => todo!(),
            crate::ast::ast_types::ExprPossibilities::Grouping(group) => {
                return self.evaluate(group.expr.as_ref());
            },
            crate::ast::ast_types::ExprPossibilities::Literal(lit) => {
                return Ok(lit.literal);
            },
            crate::ast::ast_types::ExprPossibilities::Ternary(_) => todo!(),
            crate::ast::ast_types::ExprPossibilities::Unary(unary) => {
                let right = self.evaluate(unary.right.as_ref())?;

                match unary.operator.tok {
                    TokenType::MINUS | TokenType::BANG => {
                        match right {
                            Primitive::Float(float) => return Ok(Primitive::Float(-float)),
                            Primitive::Int(int) => return Ok(Primitive::Int(-int)),
                            Primitive::String(_string) => return Err(InterpException::InvalidUnary(unary)),
                            Primitive::Bool(boolean) => return Ok(Primitive::Bool(!boolean)),
                            Primitive::None => return Ok(Primitive::Bool(true)),
                        }
                    }
                    _  => {
                        return Err(InterpException::PlaceHolder);
                    }
                }
            },
        }
    }
}