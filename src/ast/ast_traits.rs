

use super::ast_types::ExprPossibilities;

pub trait Accept<R> {
    fn accept<P: Interperable<R>>(expr: ExprPossibilities, visitor: P) -> R {
        return visitor.visit_expr(expr);
    }
}

pub trait Interperable<R> {
    fn visit_expr(&self, expr: ExprPossibilities) -> R;
}
