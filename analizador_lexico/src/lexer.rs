use crate::tokens::Token; 

const OPERATORS: [char; 3] = ['=', '+', '-'];
const PUNCTUATION: [char; 3] = [';', ',', '.'];
const DELIMITERS: [char; 4] = ['[', ']', '(', ')'];

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i32 {
        let start = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.ch == '\0' {
            return Token::EOF;
        }

        if self.ch.is_alphabetic() || self.ch == '_' {
            let ident = self.read_identifier();
            return match ident.as_str() {
                "let" => Token::KeyWord,
                _ => Token::Identifier(ident),
            };
        }

        if self.ch.is_numeric() {
            return Token::Number(self.read_number());
        }

        if OPERATORS.contains(&self.ch) {
            let operator = self.ch;
            self.read_char();
            return Token::Operator(operator);
        }

        if PUNCTUATION.contains(&self.ch) {
            let punctuation = self.ch;
            self.read_char();
            return Token::Punctuation(punctuation);
        }

        if DELIMITERS.contains(&self.ch) {
            let delimiter = self.ch;
            self.read_char();
            return Token::Delimiter(delimiter);
        }

        let illegal = self.ch;
        self.read_char();
        Token::Illegal(illegal)
    }
}
