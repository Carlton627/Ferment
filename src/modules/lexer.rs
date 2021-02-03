use crate::models::token;

extern crate regex;
use regex::Regex;

pub fn tokenizer(program: String) -> Result<Vec<token::Token>, &'static str> {
    let mut tokens: Vec<token::Token> = Vec::new();

    // TODO: make regex for: 
    //     - alphabets [case insensitive]
    //     - numbers
    //     - special characters 
    //     - keywords 

    let numbers = Regex::new(r"[0-9]").unwrap();
    let alphabets = Regex::new(r"[a-zA-Z]").unwrap();
    let operators = Regex::new(r"[=+-/*]").unwrap();
    let whitespace = Regex::new(r"[\s]").unwrap();
    let mut space_flag = 0;
    let mut word = String::new();
         
    for token in program.chars() {
        if whitespace.is_match(&token.to_string()) {
            space_flag = 1;
        } else if numbers.is_match(&token.to_string()) {
            tokens.push(token::Token {
                token_type: token::TokenType::Number,
                value: token::TokenDataType::Number(token.to_string().parse::<i32>().unwrap())
            });
        } else if alphabets.is_match(&token.to_string()) {
            
            if space_flag == 0 {
                word.push_str(&token.to_string());
            } else {
                tokens.push(token::Token {
                    token_type: token::TokenType::Character,
                    value: token::TokenDataType::Word(word)
                });
                word = String::from(token.to_string()); space_flag = 0;
            }

        } else if operators.is_match(&token.to_string()) {
            tokens.push(token::Token {
                token_type: token::TokenType::Operator,
                value: token::TokenDataType::Character(token)
            });
        } else {
            tokens.push(token::Token {
                token_type: match token {
                    '(' => token::TokenType::OpenParenthesis,
                    ')' => token::TokenType::CloseParenthesis,
                    _  => token::TokenType::SpecialChar,
                },
                value: token::TokenDataType::Character(token)
            });
        }
    }
    
    Ok(tokens)
    // Err("cant lex code")
}

