#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Assign,
    Plus,
    Semicolon,
    Identifier(String),
    Number(i32),
    EOF,
    Illegal(char),
}