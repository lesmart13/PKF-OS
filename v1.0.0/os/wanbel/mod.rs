mod lexer;
mod parser;
mod ast;
mod evaluator;

pub fn run(code: &str) {
    let tokens = lexer::tokenize(code);
    let ast = parser::parse(&tokens);
    evaluator::evaluate(&ast);
}