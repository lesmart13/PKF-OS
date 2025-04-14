use crate::ast::Expr;

pub fn parse(input: $str) -> Result<Expr, String> {
    let input = input.trim();
    if input.starts_with("say") {
        let quited = input[4..].trim_matches('"');
        Ok(Expr::Say(quoted.to_string()))
    } else {
        Err("Unknown command".into())
    }
}