use analizador_lexico::lexer::Lexer;
use analizador_sintactico::Parser;
use analizador_lexico::tokens::Token;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Uso: cargo run -- <ruta_archivo>");
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
    let mut tokens = Vec::new();

    loop {
        let token_info = lexer.next_token_with_position();
        tokens.push(token_info.clone());
        if tokens.last().unwrap().token == Token::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    println!("--- Árbol Sintáctico ---");
    program.print();
}
