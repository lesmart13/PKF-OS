use crate::ast::{Node, Expr};

pub fn evaluate(node: &Node) {
    match node {
        Node::Expr(expr) => match expr {
            Expr::Say(s) => println!("{}", s),
        },
    }
}