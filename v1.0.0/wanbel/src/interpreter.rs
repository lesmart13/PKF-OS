use std::collections::HashMap;
use crate::ast::Expr;

pub fn eval(expr: Expr) {
    use std::cell::RefCell;

    thread_local! {
        static ENV: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
        static FUN_ENV: RefCell<HashMap<String, (Vec<String>, Vec<Expr>)>> = RefCell::new(HashMap::new());
    }

    match expr {
        Expr::Say(msg) => {
            // Simple print
            println!("{}", msg);
        },

        Expr::Let(var, val) => {
            ENV.with(|env| env.borrow_mut().insert(var.clone(), val.clone()));
            println!("[stored] {} = \"{}\"", var, val);
        },

        Expr::Get(var) => {
            ENV.with(|env| {
                match env.borrow().get(&var) {
                    Some(val) => println!("{}", val),
                    None => println!("[undefined variable] {}", var),
                }
            });
        },

        Expr::IfEq(var, expected_val, then_expr) => {
            ENV.with(|env| {
                if let Some(actual_val) = env.borrow().get(&var) {
                    if actual_val == &expected_val {
                        eval(*then_expr);
                    }
                }
            });
        },

        Expr::FunDef { name, params, body } => {
            FUN_ENV.with(|fenv| {
                fenv.borrow_mut().insert(name.clone(), (params.clone(), body.clone()));
                println!("[function defined] {}", name);
            });
        },

        Expr::FunCall { name, args } => {
            let func = FUN_ENV.with(|fenv| fenv.borrow().get(&name).cloned());

            match func {
                Some((params, body)) => {
                    if args.len() != params.len() {
                        println!("[error] Argument count mismatch in call to '{}'", name);
                        return;
                    }

                    // Temporarily override ENV for local scope
                    ENV.with(|env| {
                        let mut scoped_env = env.borrow().clone();

                        for (param, arg) in params.iter().zip(args.iter()) {
                            scoped_env.insert(param.clone(), arg.clone());
                        }

                        let old_env = env.replace(scoped_env);

                        for expr in body {
                            eval(expr.clone());
                        }

                        env.replace(old_env); // Restore global state
                    });
                },
                None => {
                    println!("[error] Function '{}' not found", name);
                }
            }
        },
    }
}
