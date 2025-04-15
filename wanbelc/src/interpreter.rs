use std::collections::HashMap;
use crate::ast::Expr;

pub fn eval(expr: Expr) {
    use std::cell::RefCell;
    thread_local! {
        static ENV: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    match expr {
        Expr::Say(msg) => println!("{}", msg),
        Expr::Let(var, val) => { 
            ENV.with(|env| env.borrow_mut().insert(var.clone(), val.clone()));
            println!("[stored] {} = \"{}"\"", var, val);
        },
        Expr::Get(var) => {
            ENV.with(|env| {
                match env.borrow().get(&var) {
                    Some(val) => println!("{}", val),
                    None => println!("[undefined variable] {}", var),
                }
            });
        },
    }
}