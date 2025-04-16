mod lexer;
mod parser;
mod ast;
mod evaluator;

use std::fs;
use std::io::{self, Write};

fn main() {
    println!("Wanbel Programming Language (type 'exit' to quit)");
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        if input.ends_with(".wan") {
            // Run a .wan file
            match fs::read_to_string(input) {
                Ok(code) => run(&code),
                Err(e) => println!("Error reading file: {}", e),
            }
        } else {
            // Run code directly
            run(input);
        }
    }
}

fn run(code: &str) {
    let tokens = lexer::tokenize(code);
    let ast = parser::parse(&tokens);
    evaluator::evaluate(&ast);
}