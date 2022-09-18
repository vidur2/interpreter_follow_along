

use crate::scanner::token::Primitive;

use super::{
    ast_traits::{Accept, Interperable},
    ast_types::{Expr, ExprPossibilities},
};

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, expr: &[Option<Expr>]) -> String {
        let mut ret_str = String::new();
        ret_str.push_str(&("(".to_string() + name));
        for exp in expr.iter() {
            if let Some(exp) = exp {
                let expr_string = ExprPossibilities::accept(ExprPossibilities::Expr(exp.clone()), self);
                ret_str.push(' ');
                if let Some(expr) = expr_string {
                    ret_str.push_str(&expr);
                }
            }
        }
        ret_str.push(')');
        return ret_str;
    }
}

impl Interperable<Option<String>> for &AstPrinter {
    fn visit_expr(&self, expr: super::ast_types::ExprPossibilities) -> Option<String> {
        let mut expr_arr: [Option<Expr>; 2] = [None, None];
        match expr {
            ExprPossibilities::Binary(bin_expr) => {
                expr_arr[0] = Some(bin_expr.left);
                expr_arr[1] = Some(bin_expr.right);
                return Some(self.parenthesize(&bin_expr.operator.lexeme, &expr_arr));
            },
            ExprPossibilities::Grouping(group_expr) => {
                expr_arr[0] = Some(group_expr.expr);
                return Some(self.parenthesize("group", &expr_arr));
            },
            ExprPossibilities::Literal(lit_expr) => {
                if let Primitive::None = lit_expr.literal {
                    return Some(String::from("nil"));
                }
                return lit_expr.literal.get_value_as_str();
            },
            ExprPossibilities::Unary(unary_expr) => {
                expr_arr[0] = Some(unary_expr.right);
                return Some(self.parenthesize(&unary_expr.operator.lexeme, &expr_arr))
            },
            _ => return None,
        }
    }
}
