use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;

#[test]
fn test_lexer_basico() {
    let input = "let y = 5;";
    let mut l = Lexer::new(input);

    let tokens_esperados = vec![
        Token::KeyWord("let".to_string()),
        Token::Identifier("y".to_string()),
        Token::Operator("=".to_string()),
        Token::Number("5".to_string()),
        Token::Punctuation(';'),
        Token::EOF,
    ];

    for token_esperado in tokens_esperados {
        assert_eq!(l.next_token(), token_esperado);
    }
}

#[test]
fn test_lexer_string_char_decimal() {
    let input = "let x = \"Hola\"; y = 'c'; z = 3.14;";
    let mut l = Lexer::new(input);

    let tokens_esperados = vec![
        Token::KeyWord("let".to_string()),
        Token::Identifier("x".to_string()),
        Token::Operator("=".to_string()),
        Token::StringLiteral("Hola".to_string()),
        Token::Punctuation(';'),
        Token::Identifier("y".to_string()),
        Token::Operator("=".to_string()),
        Token::CharLiteral('c'),
        Token::Punctuation(';'),
        Token::Identifier("z".to_string()),
        Token::Operator("=".to_string()),
        Token::Number("3.14".to_string()),
        Token::Punctuation(';'),
        Token::EOF,
    ];

    for token_esperado in tokens_esperados {
        assert_eq!(l.next_token(), token_esperado);
    }
}
