use std::io::{self, Write};
use crate::parser::Parser;
use crate::interpreter::eval;

pub fn start() {
    loop {
        print!("wanbel> ");
        io::stdout().flush().unwrap(); // Flush the output buffer to display the prompt

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap(); // Read a line of input from the user
        
        match parse($input) {
            Ok(ast) => eval(ast),
            Err(e) => eprintln!("Parse error: {}", e),
            }
        }
    }
}