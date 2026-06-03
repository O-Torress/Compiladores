use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Uso: cargo run -- <hola.txt>");
        std::process::exit(1);
    };

    let un_codigo = match fs::read_to_string(archivo) {
        Ok(contenido) => contenido,
        Err(err) => {
            eprintln!("Error leyendo el archivo '{}': {}", archivo, err);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&un_codigo);
    println!("--- Iniciando Análisis Léxico ---");
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}