use crate::models::token;

extern crate regex;
use regex::Regex;

// TODO: basic syntax analysis
// TODO: re-implement the lexer to include the symbol table

pub fn lexer(program: String) -> Result<Vec<token::Token>, &'static str> {
    let mut tokens: Vec<token::Token> = Vec::new();

    // FIXME: redo the function to match correct tokens

    let number_token = Regex::new(r"[0-9]").unwrap();
    let _alphabets = Regex::new(r"[a-zA-Z]").unwrap();
    let _operators = Regex::new(r"[=+-/*]").unwrap();
    let _whitespace = Regex::new(r"[\s]").unwrap();

    // INFO: variables for letters and numbers
    let mut number = 0;

    for token in program.chars() {
        // BUG: Numbers are being matched as characters, investigate further
        if number_token.is_match(&token.to_string()) {
            number = number * 10 + token.to_string().parse::<i32>().unwrap();
            println!("{}", number);
        } else {
            // BUG: When an end of number token is reached, a number token still gets pushed on every iteration
            // because of the else block
            tokens.push(token::Token {
                token_type: token::TokenType::Number,
                value: token::TokenDataType::Number(number),
            });
            number = 0;
        }
        
    
    }
    Ok(tokens)
    // Err("cant lex code")
}


//     if alphabets.is_match(&token.to_string()) {
    //         tokens.push(token::Token {
    //             token_type: token::TokenType::Character,
    //             value: token::TokenDataType::Word(token.to_string()),
    //         });
    //     } else if operators.is_match(&token.to_string()) {
    //         tokens.push(token::Token {
    //             token_type: token::TokenType::Operator,
    //             value: token::TokenDataType::Character(token),
    //         });
    //     } else {
    //         tokens.push(token::Token {
    //             token_type: match token {
    //                 '(' => token::TokenType::LeftParenthesis,
    //                 ')' => token::TokenType::RightParenthesis,
    //                 '{' => token::TokenType::BeginBracket,
    //                 '}' => token::TokenType::EndBracket,
    //                 _ => token::TokenType::SpecialChar,
    //             },
    //             value: token::TokenDataType::Character(token),
    //         });
    //     }
