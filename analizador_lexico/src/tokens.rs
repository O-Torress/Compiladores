#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeyWord,
    Operator(char),
    Punctuation(char),
    Delimiter(char),
    Identifier(String),
    Number(i32),
    EOF,
    Illegal(char),
}