use crate::wanbel::ast::{Node, Expr};
use crate::vga::print;

pub fn evaluate(node: &Node) {
    match node {
        Node::Expr(expr) => match expr {
            Expr::Say(s) => print(&format!("{}\n", s)),
        },
    }
}