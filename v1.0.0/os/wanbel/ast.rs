#[derive(Debug)]
pub enum Expr {
    Say(String),
}

#[derive(Debug)]
pub enum Node {
    Expr(Expr),
}