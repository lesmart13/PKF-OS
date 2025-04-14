mod lexer;
mod parser;
mod ast;
mod interpreter;
mod repl;

fn main() {
    // Start the REPL (Read-Eval-Print Loop)
    repl::start_repl();
}