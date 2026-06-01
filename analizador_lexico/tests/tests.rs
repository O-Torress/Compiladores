use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::Token;

#[test]
fn test_lexer_basico() {
    let input = "let y = 5;";
    let mut l = Lexer::new(input);

    let tokens_esperados = vec![
        Token::Let,
        Token::Identifier("y".to_string()),
        Token::Assign,
        Token::Number(5),
        Token::Semicolon,
        Token::EOF,
    ];

    for token_esperado in tokens_esperados {
        assert_eq!(l.next_token(), token_esperado);
    }
}