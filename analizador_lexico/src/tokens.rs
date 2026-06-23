#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeyWord(String),
    Operator(String),
    Punctuation(char),
    Delimiter(char),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    CharLiteral(char),
    EOF,
    Illegal(char),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}