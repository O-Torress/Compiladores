#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeyWord(String),
    Operator(char),
    Punctuation(char),
    Delimiter(char),
    Identifier(String),
    Number(i32),
    EOF,
    Illegal(char),
}