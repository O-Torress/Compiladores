use std::env;
use std::fs;
use std::path::Path;

use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;
use analizador_sintactico::Parser;
use transpilador::transpiler::Transpiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Uso: cargo run -- <archivo.c>");
        std::process::exit(1);
    };

    let codigo = match fs::read_to_string(archivo) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error leyendo '{}': {}", archivo, err);
            std::process::exit(1);
        }
    };

    let codigo_limpio = preprocess(&codigo);

    let mut lexer = Lexer::new(&codigo_limpio);
    let mut tokens = Vec::new();

    loop {
        let token_info = lexer.next_token_with_position();
        let is_eof = matches!(token_info.token, Token::EOF);
        tokens.push(token_info.clone());
        if is_eof {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    if !program.errors.is_empty() {
        eprintln!("--- Errores de Sintaxis ---");
        for err in &program.errors {
            eprintln!("- {} (linea {}, columna {})", err.message, err.line, err.column);
        }
        eprintln!();
    }

    let nombre_clase = Path::new(archivo)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(capitalize)
        .unwrap_or_else(|| "Main".to_string());

    let mut transpiler = Transpiler::new(&nombre_clase);
    let java_code = transpiler.transpile(&program);

    let salida = format!("{}.java", nombre_clase);
    match fs::write(&salida, &java_code) {
        Ok(_) => println!("Transpilacion completada: {}", salida),
        Err(err) => eprintln!("Error escribiendo '{}': {}", salida, err),
    }
}

fn preprocess(input: &str) -> String {
    let mut output = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '/' && i + 1 < chars.len() {
            if chars[i + 1] == '/' {
                i += 2;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if chars[i + 1] == '*' {
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                if i + 1 < chars.len() {
                    i += 2;
                }
                continue;
            }
        }
        if chars[i] == '#' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        output.push(chars[i]);
        i += 1;
    }
    output
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}
