use crate::ast::Expr;
// use crate::parser::parse;

pub fn parse(input: &str) -> Result<Expr, String> {
    let input = input.trim();

    if input.starts_with("say ") {
        let quoted = input[4..].trim_matches('"');
        Ok(Expr::Say(quoted.to_string()))

    } else if input.starts_with("let ") {
        let parts: Vec<&str> = input[4..].splitn(2, '=').collect();
        if parts.len() == 2 {
            let var = parts[0].trim();
            let val = parts[1].trim_matches('"').trim();
            Ok(Expr::Let(var.to_string(), val.to_string()))
        } else {
            Err("Invalid let syntax".into())
        }

    } else if input.starts_with("get ") {
        let var = input[4..].trim();
        Ok(Expr::Get(var.to_string()))

    } else if input.starts_with("if ") && input.contains("==") && input.contains(" then ") {
        let condition_and_then: Vec<&str> = input[3..].splitn(2, " then ").collect();
        let condition = condition_and_then[0].trim();
        let then_part = condition_and_then[1].trim();

        let cond_parts: Vec<&str> = condition.splitn(2, "==").collect();
        if cond_parts.len() == 2 {
            let var = cond_parts[0].trim().to_string();
            let val = cond_parts[1].trim().trim_matches('"').to_string();
            let then_expr = parse(then_part)?;
            Ok(Expr::IfEq(var, val, Box::new(then_expr)))
        } else {
            Err("Invalid if condition syntax".into())
        }

    } else if input.starts_with("fun ") {
        // Example: fun greet(name) { say "Hello" }
        let open_brace = input.find('{').ok_or("Missing '{' in function")?;
        let close_brace = input.rfind('}').ok_or("Missing '}' in function")?;

        let header = &input[4..open_brace].trim(); // greet(name)
        let body_str = &input[open_brace + 1..close_brace].trim(); // say "Hello"

        let paren_start = header.find('(').ok_or("Missing '(' in function header")?;
        let paren_end = header.find(')').ok_or("Missing ')' in function header")?;

        let name = header[..paren_start].trim().to_string();
        let param_list = &header[paren_start + 1..paren_end];
        let params: Vec<String> = param_list
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        let body_lines: Vec<&str> = body_str.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        let mut body_exprs = vec![];
        for line in body_lines {
            body_exprs.push(parse(line)?);
        }

        Ok(Expr::FunDef {
            name,
            params,
            body: body_exprs,
        })

    } else if input.starts_with("call ") {
        // Example: call greet("Wanbel")
        let rest = input[5..].trim();
        let paren_start = rest.find('(').ok_or("Missing '(' in function call")?;
        let paren_end = rest.find(')').ok_or("Missing ')' in function call")?;

        let name = rest[..paren_start].trim().to_string();
        let arg_str = &rest[paren_start + 1..paren_end];
        let args: Vec<String> = arg_str
            .split(',')
            .map(|arg| arg.trim().trim_matches('"').to_string())
            .filter(|a| !a.is_empty())
            .collect();

        Ok(Expr::FunCall { name, args })

    } else {
        Err("Unknown command".into())
    }
}
