use analizador_lexico::tokens::{Token, TokenInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum DeclarationKind {
    Typed(String),
    Let,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration {
        kind: DeclarationKind,
        name: String,
        value: Option<Expression>,
        span: Span,
    },
    Expression {
        expr: Expression,
        span: Span,
    },
    If {
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
        span: Span,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
        span: Span,
    },
    DoWhile {
        body: Box<Statement>,
        condition: Expression,
        span: Span,
    },
    Return {
        value: Option<Expression>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Block {
        statements: Vec<Statement>,
        span: Span,
    },
    FunctionDefinition {
        name: String,
        return_type: String,
        parameters: Vec<(String, String)>,
        body: Box<Statement>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assignment {
        target: String,
        operator: String,
        value: Box<Expression>,
        span: Span,
    },
    Binary {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    },
    Unary {
        operator: String,
        operand: Box<Expression>,
        span: Span,
    },
    Call {
        name: String,
        arguments: Vec<Expression>,
        span: Span,
    },
    Index {
        base: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    Identifier(String, Span),
    Literal(Literal, Span),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Char(char),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Assignment { span, .. }
            | Expression::Binary { span, .. }
            | Expression::Unary { span, .. }
            | Expression::Call { span, .. }
            | Expression::Index { span, .. }
            | Expression::Identifier(_, span)
            | Expression::Literal(_, span) => *span,
        }
    }
}

impl Program {
    pub fn print(&self) {
        println!("Program");
        for statement in &self.statements {
            statement.print(1);
        }
    }
}

impl Statement {
    fn print(&self, indent: usize) {
        let padding = "  ".repeat(indent);
        match self {
            Statement::Declaration { kind, name, value, .. } => {
                let kind_label = match kind {
                    DeclarationKind::Typed(k) => format!("Declaration ({}) {}", k, name),
                    DeclarationKind::Let => format!("Declaration (let) {}", name),
                };
                println!("{}{}", padding, kind_label);
                if let Some(value) = value {
                    println!("{}  +-- Initializer", padding);
                    value.print(indent + 2);
                }
            }
            Statement::Expression { expr, .. } => {
                println!("{}Expression", padding);
                expr.print(indent + 1);
            }
            Statement::If { condition, consequence, alternative, .. } => {
                println!("{}If", padding);
                println!("{}  +-- Condition", padding);
                condition.print(indent + 2);
                println!("{}  +-- Then", padding);
                consequence.print(indent + 2);
                if let Some(alternative) = alternative {
                    println!("{}  +-- Else", padding);
                    alternative.print(indent + 2);
                }
            }
            Statement::While { condition, body, .. } => {
                println!("{}While", padding);
                println!("{}  +-- Condition", padding);
                condition.print(indent + 2);
                println!("{}  +-- Body", padding);
                body.print(indent + 2);
            }
            Statement::DoWhile { body, condition, .. } => {
                println!("{}DoWhile", padding);
                println!("{}  +-- Body", padding);
                body.print(indent + 2);
                println!("{}  +-- Condition", padding);
                condition.print(indent + 2);
            }
            Statement::Return { value, .. } => {
                println!("{}Return", padding);
                if let Some(expr) = value {
                    expr.print(indent + 1);
                }
            }
            Statement::Break { .. } => println!("{}Break", padding),
            Statement::Continue { .. } => println!("{}Continue", padding),
            Statement::Block { statements, .. } => {
                println!("{}Block", padding);
                for statement in statements {
                    statement.print(indent + 1);
                }
            }
            Statement::FunctionDefinition { name, parameters, body, .. } => {
                println!("{}Function {}", padding, name);
                for (param_name, param_type) in parameters {
                    println!("{}  +-- Param {}: {}", padding, param_name, param_type);
                }
                body.print(indent + 1);
            }
        }
    }
}

impl Expression {
    fn print(&self, indent: usize) {
        let padding = "  ".repeat(indent);
        match self {
            Expression::Assignment { target, operator, value, .. } => {
                println!("{}Assignment ({})", padding, operator);
                println!("{}  +-- Target: {}", padding, target);
                println!("{}  +-- Value", padding);
                value.print(indent + 2);
            }
            Expression::Binary { operator, left, right, .. } => {
                println!("{}Binary ({})", padding, operator);
                left.print(indent + 1);
                right.print(indent + 1);
            }
            Expression::Unary { operator, operand, .. } => {
                println!("{}Unary ({})", padding, operator);
                operand.print(indent + 1);
            }
            Expression::Call { name, arguments, .. } => {
                println!("{}Call ({})", padding, name);
                for argument in arguments {
                    argument.print(indent + 1);
                }
            }
            Expression::Index { base, index, .. } => {
                println!("{}Index", padding);
                base.print(indent + 1);
                index.print(indent + 1);
            }
            Expression::Identifier(name, _) => println!("{}Identifier ({})", padding, name),
            Expression::Literal(literal, _) => println!("{}Literal ({})", padding, literal),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "Number({})", value),
            Literal::String(value) => write!(f, "String(\"{}\")", value),
            Literal::Char(value) => write!(f, "Char('{}')", value),
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.current_token_is_eof() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
                if self.current_token_is_eof() {
                    break;
                }
                self.next_token();
            }
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let token = self.current_token().token.clone();
        match token {
            Token::KeyWord(ref kw) if kw == "if" => self.parse_if_statement(),
            Token::KeyWord(ref kw) if kw == "while" => self.parse_while_statement(),
            Token::KeyWord(ref kw) if kw == "do" => self.parse_do_while_statement(),
            Token::KeyWord(ref kw) if kw == "return" => self.parse_return_statement(),
            Token::KeyWord(ref kw) if kw == "break" => self.parse_break_statement(),
            Token::KeyWord(ref kw) if kw == "continue" => self.parse_continue_statement(),
            Token::KeyWord(ref kw) if ["int", "double", "char", "void", "let"].contains(&kw.as_str()) => {
                if self.peek2_delimiter('(') {
                    self.parse_function_definition()
                } else {
                    self.parse_declaration_statement()
                }
            }
            Token::Delimiter('{') => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_declaration_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        let kind = match &self.current_token().token {
            Token::KeyWord(kw) if kw == "let" => DeclarationKind::Let,
            Token::KeyWord(kw) => DeclarationKind::Typed(kw.clone()),
            _ => return None,
        };

        self.next_token();
        let name = self.parse_identifier()?;
        self.parse_array_suffix();
        let value = if self.current_token_is_operator("=") {
            self.next_token();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_punctuation(';');
        Some(Statement::Declaration { kind, name, value, span })
    }

    fn parse_function_definition(&mut self) -> Option<Statement> {
        let span = self.current_span();
        let return_type = match &self.current_token().token {
            Token::KeyWord(kw) => kw.clone(),
            _ => return None,
        };

        self.next_token();
        let name = self.parse_identifier()?;
        self.expect_delimiter('(')?;
        let parameters = self.parse_parameter_list();
        self.expect_delimiter(')')?;
        let body = Box::new(self.parse_statement()?);

        Some(Statement::FunctionDefinition { name, return_type, parameters, body, span })
    }

    fn parse_parameter_list(&mut self) -> Vec<(String, String)> {
        let mut parameters = Vec::new();
        if self.current_token_is_delimiter(')') {
            return parameters;
        }

        while !self.current_token_is_delimiter(')') && !self.current_token_is_eof() {
            let param_type = match &self.current_token().token {
                Token::KeyWord(kw) if ["int", "double", "char", "void"].contains(&kw.as_str()) => {
                    let ty = kw.clone();
                    self.next_token();
                    ty
                }
                _ => break,
            };

            let name = match self.parse_identifier() {
                Some(name) => name,
                None => break,
            };
            parameters.push((name, param_type));

            if self.current_token_is_punctuation(',') {
                self.next_token();
            } else {
                break;
            }
        }

        parameters
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        self.expect_delimiter('(')?;
        let condition = self.parse_expression()?;
        self.expect_delimiter(')')?;
        let consequence = Box::new(self.parse_statement()?);
        let alternative = if self.current_token_is_keyword("else") {
            self.next_token();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Some(Statement::If { condition, consequence, alternative, span })
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        self.expect_delimiter('(')?;
        let condition = self.parse_expression()?;
        self.expect_delimiter(')')?;
        let body = Box::new(self.parse_statement()?);
        Some(Statement::While { condition, body, span })
    }

    fn parse_do_while_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        let body = Box::new(self.parse_statement()?);
        self.expect_keyword("while")?;
        self.expect_delimiter('(')?;
        let condition = self.parse_expression()?;
        self.expect_delimiter(')')?;
        self.expect_punctuation(';');
        Some(Statement::DoWhile { body, condition, span })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        let value = if self.current_token_is_punctuation(';') {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect_punctuation(';');
        Some(Statement::Return { value, span })
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        self.expect_punctuation(';');
        Some(Statement::Break { span })
    }

    fn parse_continue_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.next_token();
        self.expect_punctuation(';');
        Some(Statement::Continue { span })
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        let span = self.current_span();
        self.expect_delimiter('{')?;
        let mut statements = Vec::new();
        while !self.current_token_is_delimiter('}') && !self.current_token_is_eof() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
                self.next_token();
            }
        }
        self.expect_delimiter('}')?;
        Some(Statement::Block { statements, span })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if self.current_token_is_punctuation(';') {
            self.next_token();
            return None;
        }

        let span = self.current_span();
        let expr = self.parse_expression()?;
        self.expect_punctuation(';');
        Some(Statement::Expression { expr, span })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Option<Expression> {
        let left = self.parse_logical_or()?;
        if let Token::Operator(op) = &self.current_token().token {
            if ["=", "+=", "-=", "*=", "/=", "%="].contains(&op.as_str()) {
                let span = left.span();
                let operator = op.clone();
                self.next_token();
                let right = self.parse_assignment()?;
                return match left {
                    Expression::Identifier(target, _) => Some(Expression::Assignment {
                        target,
                        operator,
                        value: Box::new(right),
                        span,
                    }),
                    _ => Some(Expression::Binary {
                        operator,
                        left: Box::new(left),
                        right: Box::new(right),
                        span,
                    }),
                };
            }
        }
        Some(left)
    }

    fn parse_logical_or(&mut self) -> Option<Expression> {
        let mut left = self.parse_logical_and()?;
        while self.current_token_is_operator("||") {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_logical_and()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_logical_and(&mut self) -> Option<Expression> {
        let mut left = self.parse_equality()?;
        while self.current_token_is_operator("&&") {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_equality()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_equality(&mut self) -> Option<Expression> {
        let mut left = self.parse_relational()?;
        while self.current_token_is_operator("==") || self.current_token_is_operator("!=") {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_relational()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_relational(&mut self) -> Option<Expression> {
        let mut left = self.parse_additive()?;
        while ["<", ">", "<=", ">="].iter().any(|op| self.current_token_is_operator(op)) {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_additive()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_additive(&mut self) -> Option<Expression> {
        let mut left = self.parse_multiplicative()?;
        while self.current_token_is_operator("+") || self.current_token_is_operator("-") {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_multiplicative()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Expression> {
        let mut left = self.parse_unary()?;
        while ["*", "/", "%"].iter().any(|op| self.current_token_is_operator(op)) {
            let operator = self.consume_operator().unwrap();
            let right = self.parse_unary()?;
            let span = left.span();
            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expression> {
        if let Token::Operator(op) = &self.current_token().token {
            if ["-", "+", "!", "&", "++", "--"].contains(&op.as_str()) {
                let span = self.current_span();
                let operator = op.clone();
                self.next_token();
                let operand = self.parse_unary()?;
                return Some(Expression::Unary { operator, operand: Box::new(operand), span });
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match &self.current_token().token {
            Token::Identifier(name) => {
                let span = self.current_span();
                let name = name.clone();
                self.next_token();

                let mut expr = Expression::Identifier(name, span);
                while self.current_token_is_delimiter('[') {
                    self.next_token();
                    let index = self.parse_expression()?;
                    self.expect_delimiter(']')?;
                    let current_span = self.current_span();
                    expr = Expression::Index { base: Box::new(expr), index: Box::new(index), span: current_span };
                }

                if self.current_token_is_delimiter('(') {
                    self.next_token();
                    let arguments = self.parse_call_arguments();
                    self.expect_delimiter(')');
                    let span = self.current_span();
                    expr = Expression::Call { name: match &expr { Expression::Identifier(name, _) => name.clone(), _ => String::new() }, arguments, span };
                }

                Some(expr)
            }
            Token::Number(value) => {
                let span = self.current_span();
                let value = value.clone();
                self.next_token();
                Some(Expression::Literal(Literal::Number(value), span))
            }
            Token::StringLiteral(value) => {
                let span = self.current_span();
                let value = value.clone();
                self.next_token();
                Some(Expression::Literal(Literal::String(value), span))
            }
            Token::CharLiteral(value) => {
                let span = self.current_span();
                let value = *value;
                self.next_token();
                Some(Expression::Literal(Literal::Char(value), span))
            }
            Token::Delimiter('(') => {
                self.next_token();
                let expression = self.parse_expression();
                self.expect_delimiter(')')?;
                expression
            }
            _ => None,
        }
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut arguments = Vec::new();
        if self.current_token_is_delimiter(')') {
            return arguments;
        }

        while !self.current_token_is_delimiter(')') && !self.current_token_is_eof() {
            if let Some(argument) = self.parse_expression() {
                arguments.push(argument);
            } else {
                break;
            }

            if self.current_token_is_punctuation(',') {
                self.next_token();
                continue;
            }
            break;
        }

        arguments
    }

    fn parse_identifier(&mut self) -> Option<String> {
        match &self.current_token().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                Some(name)
            }
            _ => None,
        }
    }

    fn parse_array_suffix(&mut self) {
        while self.current_token_is_delimiter('[') {
            self.next_token();
            if !self.current_token_is_delimiter(']') {
                let _ = self.parse_expression();
            }
            self.expect_delimiter(']');
        }
    }

    fn consume_operator(&mut self) -> Option<String> {
        if let Token::Operator(op) = &self.current_token().token {
            let operator = op.clone();
            self.next_token();
            Some(operator)
        } else {
            None
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Option<()> {
        if self.current_token_is_keyword(keyword) {
            self.next_token();
            Some(())
        } else {
            None
        }
    }

    fn expect_punctuation(&mut self, punctuation: char) -> Option<()> {
        if self.current_token_is_punctuation(punctuation) {
            self.next_token();
            Some(())
        } else {
            None
        }
    }

    fn expect_delimiter(&mut self, delimiter: char) -> Option<()> {
        if self.current_token_is_delimiter(delimiter) {
            self.next_token();
            Some(())
        } else {
            None
        }
    }

    fn current_token(&self) -> &TokenInfo {
        &self.tokens[self.position]
    }

    fn peek_token(&self) -> &TokenInfo {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            self.current_token()
        }
    }

    fn next_token(&mut self) {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
    }

    fn current_span(&self) -> Span {
        let token = self.current_token();
        Span { line: token.line, column: token.column }
    }

    fn current_token_is_eof(&self) -> bool {
        matches!(self.current_token().token, Token::EOF)
    }

    fn current_token_is_keyword(&self, keyword: &str) -> bool {
        matches!(&self.current_token().token, Token::KeyWord(value) if value == keyword)
    }

    fn current_token_is_operator(&self, operator: &str) -> bool {
        matches!(&self.current_token().token, Token::Operator(value) if value == operator)
    }

    fn current_token_is_punctuation(&self, punctuation: char) -> bool {
        matches!(&self.current_token().token, Token::Punctuation(value) if *value == punctuation)
    }

    fn current_token_is_delimiter(&self, delimiter: char) -> bool {
        matches!(&self.current_token().token, Token::Delimiter(value) if *value == delimiter)
    }

    fn peek_delimiter(&self, delimiter: char) -> bool {
        matches!(&self.peek_token().token, Token::Delimiter(value) if *value == delimiter)
    }

    fn peek2_token(&self) -> &TokenInfo {
        if self.position + 2 < self.tokens.len() {
            &self.tokens[self.position + 2]
        } else {
            self.current_token()
        }
    }

    fn peek2_delimiter(&self, delimiter: char) -> bool {
        matches!(&self.peek2_token().token, Token::Delimiter(value) if *value == delimiter)
    }
}
