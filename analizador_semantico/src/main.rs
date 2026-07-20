use analizador_semantico::analyze_code;

fn main() {
    let input = r#"
        int main() {
            int x = 5;
            int y = x + 2;
            return y;
        }
    "#;

    match analyze_code(input) {
        Ok(()) => println!("Análisis semántico correcto"),
        Err(errors) => {
            println!("Errores semánticos detectados:");
            for error in errors {
                println!("- {} (linea {}, columna {})", error.message, error.line, error.column);
            }
        }
    }
}
