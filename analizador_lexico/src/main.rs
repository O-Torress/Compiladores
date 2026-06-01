use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;

fn main() {
    let un_codigo = "int x = 10;";
    let mut lexer = Lexer::new(un_codigo);
    println!("int x = 10;");
    println!("--- Iniciando Análisis Léxico ---");
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        
        if token == Token::EOF {
            break;
        }
    }
}