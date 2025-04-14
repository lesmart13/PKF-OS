use crate::ast::Expr;

pub fn eval(expr: Expr) {
    match expr {
        Expr::Say(msg) => {
            println!("{}", msg),
        }
    }
}