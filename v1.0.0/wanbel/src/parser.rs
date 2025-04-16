use crate::ast::{Node, Expr};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Node {
    let mut index = 0;
    parse_expr(tokens, &mut index)
}

fn parse_expr(tokens: &[Token], index: &mut usize) -> Node {
    match tokens[*index] {
        Token::Say => {
            *index += 1; // Consume 'say'
            if tokens[*index] == Token::LParen {
                *index += 1; // Consume '('
                if let Token::String(ref s) = tokens[*index] {
                    let string = s.clone();
                    *index += 1; // Consume string
                    if tokens[*index] == Token::RParen {
                        *index += 1; // Consume ')'
                        Node::Expr(Expr::Say(string))
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected string");
                }
            } else {
                panic!("Expected '('");
            }
        }
        _ => panic!("Unexpected token: {:?}", tokens[*index]),
    }
}