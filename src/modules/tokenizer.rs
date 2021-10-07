use crate::models::token;

extern crate regex;
use regex::Regex;

// TODO: basic syntax analysis
// TODO: re-implement the lexer to include the symbol table

pub fn lexer(program: String) -> Result<Vec<token::Token>, &'static str> {
    let mut tokens: Vec<token::Token> = Vec::new();

    let numbers = Regex::new(r"[0-9]").unwrap();
    let alphabets = Regex::new(r"[a-zA-Z]").unwrap();
    let operators = Regex::new(r"[=+-/*]").unwrap();
    let _whitespace = Regex::new(r"[\s]").unwrap();
    for token in program.chars() {
        if numbers.is_match(&token.to_string()) {
            tokens.push(token::Token {
                token_type: token::TokenType::Number,
                value: token::TokenDataType::Number(token.to_string().parse::<i32>().unwrap()),
            });
        } else if alphabets.is_match(&token.to_string()) {
            tokens.push(token::Token {
                token_type: token::TokenType::Character,
                value: token::TokenDataType::Word(token.to_string()),
            });
        } else if operators.is_match(&token.to_string()) {
            tokens.push(token::Token {
                token_type: token::TokenType::Operator,
                value: token::TokenDataType::Character(token),
            });
        } else {
            tokens.push(token::Token {
                token_type: match token {
                    '(' => token::TokenType::OpenParenthesis,
                    ')' => token::TokenType::CloseParenthesis,
                    '{' => token::TokenType::BeginBracket,
                    '}' => token::TokenType::EndBracket,
                    _ => token::TokenType::SpecialChar,
                },
                value: token::TokenDataType::Character(token),
            });
        }
    }
    Ok(tokens)
    // Err("cant lex code")
}
