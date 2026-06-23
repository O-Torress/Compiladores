use crate::tokens::Token; 

const OPERATORS: [char; 11] = ['=', '+', '-', '*', '/', '%', '!', '<', '>', '&', '|'];
const PUNCTUATION: [char; 4] = [';', ',', '.', ':'];
const DELIMITERS: [char; 6] = ['[', ']', '(', ')','{','}'];

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
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

        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        let mut has_dot = false;

        while self.ch.is_numeric() || (self.ch == '.' && !has_dot) {
            if self.ch == '.' {
                has_dot = true;
            }
            self.read_char();
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let start = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let value: String = self.input[start..self.position].iter().collect();
        if self.ch == '"' {
            self.read_char();
        }
        value
    }

    fn read_char_literal(&mut self) -> Option<char> {
        self.read_char();

        let ch = if self.ch == '\\' {
            self.read_char();
            match self.ch {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                '0' => '\0',
                other => other,
            }
        } else {
            self.ch
        };

        self.read_char();
        if self.ch == '\'' {
            self.read_char();
            Some(ch)
        } else {
            None
        }
    }

    fn lookup_ident(&self, ident: String) -> Token {
        match ident.as_str() {
            "void" | "int" | "double" | "char" | "if" | "else" | "while" | "return" | "let" | "do" => Token::KeyWord(ident),
            _ => Token::Identifier(ident),
        }
    }

    fn next_token_inner(&mut self) -> Token {
        self.skip_whitespace();

        if self.ch == '\0' {
            return Token::EOF;
        }

        if self.ch.is_alphabetic() || self.ch == '_' {
            let ident = self.read_identifier();
            return self.lookup_ident(ident);
        }

        if self.ch == '"' {
            return Token::StringLiteral(self.read_string());
        }

        if self.ch == '\'' {
            return match self.read_char_literal() {
                Some(c) => Token::CharLiteral(c),
                None => Token::Illegal('\''),
            };
        }

        if self.ch.is_numeric() {
            return Token::Number(self.read_number());
        }

        if OPERATORS.contains(&self.ch) {
            let operator = self.ch;
            let token = match (self.ch, self.peek_char()) {
                ('=', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("==".to_string())
                }
                ('!', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("!=".to_string())
                }
                ('<', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("<=".to_string())
                }
                ('>', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator(">=".to_string())
                }
                ('&', '&') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("&&".to_string())
                }
                ('|', '|') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("||".to_string())
                }
                ('+', '+') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("++".to_string())
                }
                ('-', '-') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("--".to_string())
                }
                ('+', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("+=".to_string())
                }
                ('-', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("-=".to_string())
                }
                ('*', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("*=".to_string())
                }
                ('/', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("/=".to_string())
                }
                ('%', '=') => {
                    self.read_char();
                    self.read_char();
                    Token::Operator("%=".to_string())
                }
                _ => {
                    self.read_char();
                    Token::Operator(operator.to_string())
                }
            };
            return token;
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

    pub fn next_token(&mut self) -> Token {
        self.next_token_with_position().token
    }

    pub fn next_token_with_position(&mut self) -> crate::tokens::TokenInfo {
        self.skip_whitespace();
        let start_line = self.line;
        let start_column = self.column;

        let token = self.next_token_inner();
        crate::tokens::TokenInfo {
            token,
            line: start_line,
            column: start_column,
        }
    }
}
