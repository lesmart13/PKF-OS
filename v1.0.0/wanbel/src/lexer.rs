#[derive(Debug, PartialEq)]
pub enum Token {
    Say,           // 'say'
    LParen,        // '('
    RParen,        // ')'
    String(String),// 'string'
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '\'' => {
                chars.next(); // Skip opening quote
                current.clear();
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next(); // Skip closing quote
                        tokens.push(Token::String(current.clone()));
                        current.clear();
                        break;
                    }
                    current.push(c);
                    chars.next();
                }
            }
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            _ => {
                current.clear();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '(' || c == ')' || c == '\'' {
                        break;
                    }
                    current.push(c);
                    chars.next();
                }
                if current == "say" {
                    tokens.push(Token::Say);
                }
            }
        }
    }

    tokens.push(Token::EOF);
    tokens
}