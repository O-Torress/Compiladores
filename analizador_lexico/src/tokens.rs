#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeyWord,
    AssignOperator,
    Plus,
    Semicolon,
    Identifier(String),
    Number(i32),
    EOF,
    Illegal(char),
}