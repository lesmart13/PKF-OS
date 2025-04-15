#[derive(Debug)]
pub enum Expr {
    Say(String),
    Let(Sting, String),
    Get(String),
    IfEq(String, String, Box<Expr>),
    // FunDef(String, Vec<String>, Vec<Expr>),
    // FunCall(String, Vec<String>),
    FunDef {
        name: String, 
        params: Vec<String>,
        body: Vec<Expr>,
    },
    FunCall {
        name: String,
        args: Vec<String>,
    },
}