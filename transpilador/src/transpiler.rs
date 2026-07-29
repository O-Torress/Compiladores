use analizador_sintactico::parser::{
    DeclarationKind, Expression, Literal, Program, Statement,
};

pub struct Transpiler {
    output: String,
    indent: usize,
    needs_scanner: bool,
    class_name: String,
    is_main_function: bool,
}

impl Transpiler {
    pub fn new(class_name: &str) -> Self {
        Transpiler {
            output: String::new(),
            indent: 0,
            needs_scanner: false,
            class_name: class_name.to_string(),
            is_main_function: false,
        }
    }

    pub fn transpile(&mut self, program: &Program) -> String {
        self.output.clear();
        self.needs_scanner = self.detect_scanf_usage(program);

        self.emit_line("// Generado por transpilador - C-like a Java");
        self.emit_line("");

        if self.needs_scanner {
            self.emit_line("import java.util.Scanner;");
            self.emit_line("");
        }

        self.emit(&format!("public class {}", self.class_name));
        self.emit(" {\n");
        self.indent = 1;

        if self.needs_scanner {
            self.emit_line(
                "private static Scanner scanner = new Scanner(System.in);",
            );
            self.emit_line("");
        }

        for statement in &program.statements {
            self.transpile_statement(statement);
        }

        self.indent = 0;
        self.emit_line("}");
        self.output.clone()
    }

    fn detect_scanf_usage(&self, program: &Program) -> bool {
        self.scan_for_scanf(&program.statements)
    }

    fn scan_for_scanf(&self, statements: &[Statement]) -> bool {
        for stmt in statements {
            match stmt {
                Statement::Expression { expr, .. } => {
                    if self.expr_contains_scanf(expr) {
                        return true;
                    }
                }
                Statement::If {
                    condition,
                    consequence,
                    alternative,
                    ..
                } => {
                    if self.expr_contains_scanf(condition)
                        || self
                            .scan_for_scanf(&[consequence.as_ref().clone()])
                        || alternative.as_ref().is_some_and(|a| {
                            self.scan_for_scanf(&[a.as_ref().clone()])
                        })
                    {
                        return true;
                    }
                }
                Statement::While { condition, body, .. } => {
                    if self.expr_contains_scanf(condition)
                        || self.scan_for_scanf(&[body.as_ref().clone()])
                    {
                        return true;
                    }
                }
                Statement::DoWhile { body, condition, .. } => {
                    if self.expr_contains_scanf(condition)
                        || self.scan_for_scanf(&[body.as_ref().clone()])
                    {
                        return true;
                    }
                }
                Statement::Block { statements, .. } => {
                    if self.scan_for_scanf(statements) {
                        return true;
                    }
                }
                Statement::FunctionDefinition { body, .. } => {
                    if let Statement::Block { statements, .. } = body.as_ref() {
                        if self.scan_for_scanf(statements) {
                            return true;
                        }
                    } else if self.scan_for_scanf(&[body.as_ref().clone()]) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn expr_contains_scanf(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Call { name, arguments, .. } => {
                if name == "scanf" {
                    return true;
                }
                for arg in arguments {
                    if self.expr_contains_scanf(arg) {
                        return true;
                    }
                }
                false
            }
            Expression::Assignment { value, .. } => self.expr_contains_scanf(value),
            Expression::Binary { left, right, .. } => {
                self.expr_contains_scanf(left) || self.expr_contains_scanf(right)
            }
            Expression::Unary { operand, .. } => self.expr_contains_scanf(operand),
            Expression::Index { base, index, .. } => {
                self.expr_contains_scanf(base) || self.expr_contains_scanf(index)
            }
            _ => false,
        }
    }

    fn emit(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn emit_indent(&mut self) {
        self.emit(&"    ".repeat(self.indent));
    }

    fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.emit(s);
        self.emit("\n");
    }

    fn transpile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Declaration {
                kind, name, value, ..
            } => {
                let java_type = match kind {
                    DeclarationKind::Typed(ty) => self.c_type_to_java(ty),
                    DeclarationKind::Let => "var",
                };
                self.emit_indent();
                self.emit(&format!("{} {}", java_type, name));
                if let Some(expr) = value {
                    self.emit(" = ");
                    self.transpile_expression(expr);
                }
                self.emit(";\n");
            }
            Statement::Expression { expr, .. } => {
                if let Expression::Call {
                    name, arguments, ..
                } = expr
                {
                    if name == "printf" || name == "puts" || name == "putchar" {
                        self.transpile_builtin_call(name, arguments);
                        return;
                    }
                }
                self.emit_indent();
                self.transpile_expression(expr);
                self.emit(";\n");
            }
            Statement::If {
                condition,
                consequence,
                alternative,
                ..
            } => {
                self.transpile_if(condition, consequence, alternative);
            }
            Statement::While { condition, body, .. } => {
                self.emit_indent();
                self.emit("while (");
                self.transpile_expression(condition);
                self.emit(") ");
                self.transpile_block_open(body);
                self.emit("}\n");
            }
            Statement::DoWhile { body, condition, .. } => {
                self.emit_line("do ");
                self.transpile_block_open(body);
                self.emit_indent();
                self.emit("} while (");
                self.transpile_expression(condition);
                self.emit(");\n");
            }
            Statement::Return { value, .. } => {
                self.emit_indent();
                if self.is_main_function {
                    if let Some(expr) = value {
                        self.emit("System.exit(");
                        self.transpile_expression(expr);
                        self.emit(");\n");
                    } else {
                        self.emit("return;\n");
                    }
                } else if let Some(expr) = value {
                    self.emit("return ");
                    self.transpile_expression(expr);
                    self.emit(";\n");
                } else {
                    self.emit("return;\n");
                }
            }
            Statement::Break { .. } => {
                self.emit_line("break;");
            }
            Statement::Continue { .. } => {
                self.emit_line("continue;");
            }
            Statement::Block {
                statements, ..
            } => {
                self.emit_line("{");
                self.indent += 1;
                for stmt in statements {
                    self.transpile_statement(stmt);
                }
                self.indent -= 1;
                self.emit_line("}");
            }
            Statement::FunctionDefinition {
                name,
                return_type,
                parameters,
                body,
                ..
            } => {
                let is_main = name == "main";
                self.is_main_function = is_main;

                self.emit_indent();
                if is_main {
                    self.emit("public static void main(String[] args)");
                } else {
                    let java_ret = self.c_type_to_java(return_type);
                    let params: Vec<String> = parameters
                        .iter()
                        .map(|(pname, ptype)| {
                            format!("{} {}", self.c_type_to_java(ptype), pname)
                        })
                        .collect();
                    self.emit(&format!(
                        "public static {} {}({})",
                        java_ret,
                        name,
                        params.join(", ")
                    ));
                }

                self.emit(" ");
                self.transpile_block_open(body);
                self.emit_line("}");
            }
        }
    }

    fn transpile_block_open(&mut self, stmt: &Statement) {
        if let Statement::Block { statements, .. } = stmt {
            self.emit("{\n");
            self.indent += 1;
            for s in statements {
                self.transpile_statement(s);
            }
            self.indent -= 1;
        } else {
            self.emit("{\n");
            self.indent += 1;
            self.transpile_statement(stmt);
            self.indent -= 1;
        }
    }

    fn transpile_if(
        &mut self,
        condition: &Expression,
        consequence: &Statement,
        alternative: &Option<Box<Statement>>,
    ) {
        self.emit_indent();
        self.emit("if (");
        self.transpile_expression(condition);
        self.emit(") ");
        self.transpile_block_open(consequence);
        self.transpile_else_chain(alternative);
    }

    fn transpile_else_chain(&mut self, alternative: &Option<Box<Statement>>) {
        match alternative {
            Some(alt) if matches!(alt.as_ref(), Statement::If { .. }) => {
                if let Statement::If {
                    condition,
                    consequence,
                    alternative,
                    ..
                } = alt.as_ref()
                {
                    self.emit_indent();
                    self.emit("} else if (");
                    self.transpile_expression(condition);
                    self.emit(") ");
                    self.transpile_block_open(consequence);
                    self.transpile_else_chain(alternative);
                }
            }
            Some(alt) => {
                self.emit_indent();
                self.emit("} else ");
                self.transpile_block_open(alt);
                self.emit_indent();
                self.emit("}\n");
            }
            None => {
                self.emit_line("}");
            }
        }
    }

    fn transpile_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Assignment {
                target,
                operator,
                value,
                ..
            } => {
                self.emit(target);
                self.emit(&format!(" {} ", operator));
                self.transpile_expression(value);
            }
            Expression::Binary {
                operator,
                left,
                right,
                ..
            } => {
                let needs_parens = matches!(
                    operator.as_str(),
                    "+" | "-" | "*" | "/" | "%" | "&&" | "||" | "==" | "!=" | "<" | ">"
                        | "<=" | ">="
                );
                if needs_parens {
                    self.emit("(");
                }
                self.transpile_expression(left);
                self.emit(&format!(" {} ", operator));
                self.transpile_expression(right);
                if needs_parens {
                    self.emit(")");
                }
            }
            Expression::Unary { operator, operand, .. } => {
                if operator == "&" {
                    self.transpile_expression(operand);
                } else {
                    self.emit(operator);
                    self.transpile_expression(operand);
                }
            }
            Expression::Call { name, arguments, .. } => {
                if self.is_builtin_function(name) {
                    self.transpile_builtin_call_expr(name, arguments);
                } else {
                    self.emit(&format!("{}(", name));
                    for (i, arg) in arguments.iter().enumerate() {
                        if i > 0 {
                            self.emit(", ");
                        }
                        self.transpile_expression(arg);
                    }
                    self.emit(")");
                }
            }
            Expression::Index { base, index, .. } => {
                self.transpile_expression(base);
                self.emit("[");
                self.transpile_expression(index);
                self.emit("]");
            }
            Expression::Identifier(name, _) => {
                self.emit(name);
            }
            Expression::Literal(lit, _) => match lit {
                Literal::Number(val) => self.emit(val),
                Literal::String(val) => self.emit(&format!("\"{}\"", val)),
                Literal::Char(val) => self.emit(&format!("'{}'", escape_char(*val))),
            },
        }
    }

    fn c_type_to_java<'a>(&self, ty: &'a str) -> &'a str {
        match ty {
            "int" => "int",
            "double" => "double",
            "char" => "char",
            "void" => "void",
            "float" => "float",
            "long" => "long",
            "short" => "short",
            _ => ty,
        }
    }

    fn is_builtin_function(&self, name: &str) -> bool {
        matches!(
            name,
            "printf" | "scanf" | "puts" | "putchar" | "getchar" | "strlen" | "atoi"
                | "atof" | "fmod" | "malloc" | "free"
        )
    }

    fn transpile_builtin_call(&mut self, name: &str, args: &[Expression]) {
        match name {
            "printf" => {
                self.emit_indent();
                self.emit("System.out.printf(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.transpile_expression(arg);
                }
                self.emit(");\n");
            }
            "puts" => {
                self.emit_indent();
                self.emit("System.out.println(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(");\n");
            }
            "putchar" => {
                self.emit_indent();
                self.emit("System.out.print(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(");\n");
            }
            "scanf" => {
                self.transpile_scanf(args);
            }
            _ => {
                self.emit_indent();
                self.emit(&format!("{}(", name));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.transpile_expression(arg);
                }
                self.emit(");\n");
            }
        }
    }

    fn transpile_builtin_call_expr(&mut self, name: &str, args: &[Expression]) {
        match name {
            "printf" => {
                self.emit("System.out.printf(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
            "puts" => {
                self.emit("System.out.println(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
            "putchar" => {
                self.emit("System.out.print(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
            "getchar" => {
                self.emit("(char)System.in.read()");
            }
            "strlen" => {
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                    self.emit(".length()");
                }
            }
            "atoi" => {
                self.emit("Integer.parseInt(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
            "atof" => {
                self.emit("Double.parseDouble(");
                if let Some(arg) = args.first() {
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
            "fmod" => {
                if args.len() >= 2 {
                    self.emit("(");
                    self.transpile_expression(&args[0]);
                    self.emit(" % ");
                    self.transpile_expression(&args[1]);
                    self.emit(")");
                }
            }
            "malloc" => {
                if let Some(arg) = args.first() {
                    self.emit("new byte[");
                    self.transpile_expression(arg);
                    self.emit("]");
                }
            }
            "free" => {
                self.emit("/* free */");
            }
            "scanf" => {
                self.transpile_scanf_expr(args);
            }
            _ => {
                self.emit(&format!("{}(", name));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.transpile_expression(arg);
                }
                self.emit(")");
            }
        }
    }

    fn transpile_scanf_expr(&mut self, args: &[Expression]) {
        let fmt = if !args.is_empty() {
            if let Expression::Literal(Literal::String(s), _) = &args[0] {
                s.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let specs = self.parse_scan_format(&fmt);

        if args.len() - 1 != specs.len() {
            self.emit("/* scanf: format mismatch */ 0");
            return;
        }

        for i in 0..specs.len() {
            if i > 0 {
                self.emit("; ");
            }
            let var_expr = &args[i + 1];
            let var_name = match var_expr {
                Expression::Unary { operand, .. } => match operand.as_ref() {
                    Expression::Identifier(name, _) => name.clone(),
                    _ => String::new(),
                },
                Expression::Identifier(name, _) => name.clone(),
                _ => String::new(),
            };

            match specs[i] {
                'd' | 'i' => {
                    self.emit(&format!("{} = scanner.nextInt()", var_name))
                }
                'f' | 'g' | 'e' | 'l' => {
                    self.emit(&format!("{} = scanner.nextDouble()", var_name))
                }
                'c' => self.emit(&format!(
                    "{} = scanner.next().charAt(0)",
                    var_name
                )),
                's' => self.emit(&format!("{} = scanner.next()", var_name)),
                _ => self.emit(&format!("{} = scanner.next()", var_name)),
            }
        }
    }

    fn transpile_scanf(&mut self, args: &[Expression]) {
        let fmt = if let Some(Expression::Literal(Literal::String(s), _)) = args.first()
        {
            s.clone()
        } else {
            String::new()
        };

        let specs = self.parse_scan_format(&fmt);

        let vars: Vec<String> = args[1..]
            .iter()
            .map(|arg| match arg {
                Expression::Unary { operand, .. } => match operand.as_ref() {
                    Expression::Identifier(name, _) => name.clone(),
                    _ => String::new(),
                },
                Expression::Identifier(name, _) => name.clone(),
                _ => String::new(),
            })
            .collect();

        if vars.len() != specs.len() {
            self.emit_line(
                "// scanf: error de formato, revisar manualmente",
            );
            return;
        }

        self.emit_indent();
        self.emit("// scanf:");
        for (i, spec) in specs.iter().enumerate() {
            let var_name = &vars[i];
            match spec {
                'd' | 'i' => {
                    self.emit(&format!(" {} = scanner.nextInt();", var_name))
                }
                'f' | 'g' | 'e' => {
                    self.emit(&format!(
                        " {} = scanner.nextDouble();",
                        var_name
                    ))
                }
                'l' => self.emit(&format!(
                    " {} = scanner.nextDouble();",
                    var_name
                )),
                'c' => self.emit(&format!(
                    " {} = scanner.next().charAt(0);",
                    var_name
                )),
                's' => {
                    self.emit(&format!(" {} = scanner.next();", var_name))
                }
                _ => {
                    self.emit(&format!(" {} = scanner.next();", var_name))
                }
            }
        }
        self.emit("\n");
    }

    fn parse_scan_format(&self, fmt: &str) -> Vec<char> {
        let mut specs = Vec::new();
        let chars: Vec<char> = fmt.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '%' {
                i += 1;
                if i < chars.len() {
                    if chars[i] == '%' {
                        i += 1;
                        continue;
                    }
                    while i < chars.len()
                        && "*0123456789.".contains(chars[i])
                    {
                        i += 1;
                    }
                    if i < chars.len() && chars[i] != ' ' {
                        specs.push(chars[i]);
                        i += 1;
                    }
                }
            } else {
                i += 1;
            }
        }
        specs
    }
}

fn escape_char(c: char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '\"' => "\\\"".to_string(),
        '\0' => "\\0".to_string(),
        _ => c.to_string(),
    }
}
