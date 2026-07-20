use std::{env, fs};

use analizador_semantico::analyze_code;

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

    match analyze_code(&input) {
        Ok(()) => println!("Análisis semántico correcto"),
        Err(errors) => {
            println!("Errores semánticos detectados:");
            for error in errors {
                println!("- {} (linea {}, columna {})", error.message, error.line, error.column);
            }
        }
    }
}
