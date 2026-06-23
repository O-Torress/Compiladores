use analizador_lexico::lexer::Lexer;
use analizador_lexico::tokens::{Token, TokenInfo};

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
fn test_lexer_token_positions() {
    let input = "let x = 1;\n  y = 2;";
    let mut l = Lexer::new(input);

    let tokens_esperados = vec![
        TokenInfo { token: Token::KeyWord("let".to_string()), line: 1, column: 1 },
        TokenInfo { token: Token::Identifier("x".to_string()), line: 1, column: 5 },
        TokenInfo { token: Token::Operator("=".to_string()), line: 1, column: 7 },
        TokenInfo { token: Token::Number("1".to_string()), line: 1, column: 9 },
        TokenInfo { token: Token::Punctuation(';'), line: 1, column: 10 },
        TokenInfo { token: Token::Identifier("y".to_string()), line: 2, column: 3 },
        TokenInfo { token: Token::Operator("=".to_string()), line: 2, column: 5 },
        TokenInfo { token: Token::Number("2".to_string()), line: 2, column: 7 },
        TokenInfo { token: Token::Punctuation(';'), line: 2, column: 8 },
        TokenInfo { token: Token::EOF, line: 2, column: 9 },
    ];

    for expected in tokens_esperados {
        assert_eq!(l.next_token_with_position(), expected);
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
