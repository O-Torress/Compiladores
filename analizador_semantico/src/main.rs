use std::{env, fs};

use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;
use analizador_semantico::analyze_program;
use analizador_sintactico::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <archivo.c>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let input = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("No se pudo leer el archivo '{}': {}", file_path, err);
        std::process::exit(2);
    });

    let mut lexer = Lexer::new(&input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token_with_position();
        tokens.push(token.clone());
        if matches!(token.token, Token::EOF) {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    println!("--- Árbol Sintáctico ---");
    program.print();

    println!("\n--- Análisis Semántico ---");
    match analyze_program(&program) {
        Ok(()) => println!("Análisis semántico correcto"),
        Err(errors) => {
            println!("Errores semánticos detectados:");
            for error in errors {
                println!("- {} (linea {}, columna {})", error.message, error.line, error.column);
            }
        }
    }
}
