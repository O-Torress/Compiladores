use std::collections::HashMap;

use analizador_lexico::lexer::Lexer;
use analizador_sintactico::parser::{Expression, Literal, Program, Statement, DeclarationKind};

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Int,
    Double,
    Char,
    Void,
    String,
    Bool,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: PrimitiveType,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default)]
pub struct SemanticAnalyzer {
    errors: Vec<SemanticError>,
    scopes: Vec<HashMap<String, Symbol>>,
    functions: HashMap<String, FunctionSignature>,
    current_function: Option<FunctionSignature>,
    loop_depth: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: PrimitiveType,
    pub parameters: Vec<(String, PrimitiveType)>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        self.enter_scope();
        self.register_builtin_functions();
        self.analyze_program(program);
        self.leave_scope();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn analyze_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Declaration { kind, name, value } => {
                let declared_type = match kind {
                    DeclarationKind::Typed(ty) => self.parse_type(ty),
                    DeclarationKind::Let => PrimitiveType::Unknown,
                };

                if self.current_scope().contains_key(name) {
                    self.error(format!("La variable '{}' ya está declarada en este ámbito", name), 1, 1);
                } else {
                    let symbol = Symbol {
                        name: name.clone(),
                        kind: SymbolKind::Variable,
                        ty: declared_type.clone(),
                        line: 1,
                        column: 1,
                    };
                    self.current_scope_mut().insert(name.clone(), symbol);
                }

                if let Some(value_expr) = value {
                    let inferred = self.infer_expression_type(value_expr);
                    if declared_type != PrimitiveType::Unknown && inferred != PrimitiveType::Unknown {
                        if !self.types_are_compatible(&declared_type, &inferred) {
                            self.error(format!("No se puede asignar un valor de tipo {:?} a '{}' de tipo {:?}", inferred, name, declared_type), 1, 1);
                        }
                    }
                }
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            Statement::If { condition, consequence, alternative } => {
                self.ensure_boolean_condition(condition);
                self.enter_scope();
                self.analyze_statement(consequence);
                self.leave_scope();

                if let Some(alt) = alternative {
                    self.enter_scope();
                    self.analyze_statement(alt);
                    self.leave_scope();
                }
            }
            Statement::While { condition, body } => {
                self.ensure_boolean_condition(condition);
                self.loop_depth += 1;
                self.analyze_statement(body);
                self.loop_depth -= 1;
            }
            Statement::DoWhile { body, condition } => {
                self.loop_depth += 1;
                self.analyze_statement(body);
                self.loop_depth -= 1;
                self.ensure_boolean_condition(condition);
            }
            Statement::Return(expr) => {
                let expr_type = expr.as_ref().map(|e| self.infer_expression_type(e)).unwrap_or(PrimitiveType::Void);
                if let Some(current_function) = &self.current_function {
                    if !self.types_are_compatible(&current_function.return_type, &expr_type) {
                        self.error(format!("El retorno no coincide con el tipo esperado {:?}", current_function.return_type), 1, 1);
                    }
                }
            }
            Statement::Break => {
                if self.loop_depth == 0 {
                    self.error("'break' solo puede usarse dentro de un bucle".to_string(), 1, 1);
                }
            }
            Statement::Continue => {
                if self.loop_depth == 0 {
                    self.error("'continue' solo puede usarse dentro de un bucle".to_string(), 1, 1);
                }
            }
            Statement::Block(statements) => {
                self.enter_scope();
                for stmt in statements {
                    self.analyze_statement(stmt);
                }
                self.leave_scope();
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> PrimitiveType {
        match expr {
            Expression::Assignment { target, value, .. } => {
                let target_type = self.lookup_variable_type(target);
                let value_type = self.analyze_expression(value);
                if !self.types_are_compatible(&target_type, &value_type) {
                    self.error(format!("No se puede asignar un valor de tipo {:?} a '{}'", value_type, target), 1, 1);
                }
                target_type
            }
            Expression::Binary { operator, left, right } => {
                let left_type = self.analyze_expression(left);
                let right_type = self.analyze_expression(right);
                self.check_binary_operation(operator, &left_type, &right_type)
            }
            Expression::Unary { operator: _, operand } => self.analyze_expression(operand),
            Expression::Call { name, arguments } => {
                self.validate_function_call(name, arguments)
            }
            Expression::Identifier(name) => self.lookup_variable_type(name),
            Expression::Literal(lit) => self.literal_type(lit),
        }
    }

    fn infer_expression_type(&mut self, expr: &Expression) -> PrimitiveType {
        self.analyze_expression(expr)
    }

    fn validate_function_call(&mut self, name: &str, arguments: &[Expression]) -> PrimitiveType {
        if let Some(signature) = self.functions.get(name).cloned() {
            if signature.parameters.len() != arguments.len() {
                self.error(
                    format!("La función '{}' espera {} argumentos, recibió {}", name, signature.parameters.len(), arguments.len()),
                    1,
                    1,
                );
                return signature.return_type.clone();
            }

            for (index, arg) in arguments.iter().enumerate() {
                let arg_type = self.analyze_expression(arg);
                let expected_type = &signature.parameters[index].1;
                if !self.types_are_compatible(expected_type, &arg_type) {
                    self.error(
                        format!("El argumento {} de '{}' no coincide con el tipo esperado {:?}", index + 1, name, expected_type),
                        1,
                        1,
                    );
                }
            }

            signature.return_type.clone()
        } else {
            self.error(format!("La función '{}' no está definida", name), 1, 1);
            PrimitiveType::Unknown
        }
    }

    fn check_binary_operation(&mut self, operator: &str, left: &PrimitiveType, right: &PrimitiveType) -> PrimitiveType {
        match operator {
            "+" | "-" | "*" | "/" | "%" => {
                if left == right {
                    left.clone()
                } else {
                    self.error(format!("La operación '{}' no es válida para tipos {:?} y {:?}", operator, left, right), 1, 1);
                    PrimitiveType::Unknown
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                if left == right {
                    PrimitiveType::Bool
                } else {
                    self.error(format!("La comparación '{}' no es válida para tipos {:?} y {:?}", operator, left, right), 1, 1);
                    PrimitiveType::Bool
                }
            }
            "&&" | "||" => {
                if left == &PrimitiveType::Bool && right == &PrimitiveType::Bool {
                    PrimitiveType::Bool
                } else {
                    self.error(format!("La operación lógica '{}' requiere booleanos", operator), 1, 1);
                    PrimitiveType::Bool
                }
            }
            _ => PrimitiveType::Unknown,
        }
    }

    fn ensure_boolean_condition(&mut self, expr: &Expression) {
        let ty = self.analyze_expression(expr);
        if ty != PrimitiveType::Bool {
            self.error("La condición debe evaluar a un valor booleano".to_string(), 1, 1);
        }
    }

    fn lookup_variable_type(&mut self, name: &str) -> PrimitiveType {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return symbol.ty.clone();
            }
        }
        self.error(format!("La variable '{}' no está declarada", name), 1, 1);
        PrimitiveType::Unknown
    }

    fn parse_type(&self, token: &str) -> PrimitiveType {
        match token {
            "int" => PrimitiveType::Int,
            "double" => PrimitiveType::Double,
            "char" => PrimitiveType::Char,
            "void" => PrimitiveType::Void,
            "string" => PrimitiveType::String,
            _ => PrimitiveType::Unknown,
        }
    }

    fn literal_type(&self, literal: &Literal) -> PrimitiveType {
        match literal {
            Literal::Number(_) => PrimitiveType::Int,
            Literal::String(_) => PrimitiveType::String,
            Literal::Char(_) => PrimitiveType::Char,
        }
    }

    fn types_are_compatible(&self, declared: &PrimitiveType, inferred: &PrimitiveType) -> bool {
        if declared == inferred {
            return true;
        }

        match (declared, inferred) {
            (PrimitiveType::Int, PrimitiveType::Double) => true,
            (PrimitiveType::Double, PrimitiveType::Int) => true,
            _ => false,
        }
    }

    fn register_builtin_functions(&mut self) {
        self.functions.insert(
            "printf".to_string(),
            FunctionSignature {
                name: "printf".to_string(),
                return_type: PrimitiveType::Int,
                parameters: vec![("format".to_string(), PrimitiveType::String)],
            },
        );
    }

    fn error(&mut self, message: String, line: usize, column: usize) {
        self.errors.push(SemanticError { message, line, column });
    }

    fn current_scope(&self) -> &HashMap<String, Symbol> {
        self.scopes.last().expect("No scope available")
    }

    fn current_scope_mut(&mut self) -> &mut HashMap<String, Symbol> {
        self.scopes.last_mut().expect("No scope available")
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }
}

pub fn analyze_code(input: &str) -> Result<(), Vec<SemanticError>> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token_with_position();
        tokens.push(token.clone());
        if matches!(token.token, analizador_lexico::tokens::Token::EOF) {
            break;
        }
    }

    let mut parser = analizador_sintactico::parser::Parser::new(tokens);
    let program = parser.parse_program();
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_assignments_and_returns() {
        let input = r#"
            int main() {
                int x = 5;
                int y = x + 2;
                return y;
            }
        "#;

        assert!(analyze_code(input).is_ok());
    }

    #[test]
    fn detects_undeclared_variables() {
        let input = r#"
            int main() {
                int x = y;
                return x;
            }
        "#;

        let result = analyze_code(input).unwrap_err();
        assert!(!result.is_empty());
    }

    #[test]
    fn detects_break_outside_loop() {
        let input = r#"
            int main() {
                break;
                return 0;
            }
        "#;

        let result = analyze_code(input).unwrap_err();
        assert!(!result.is_empty());
    }
}
