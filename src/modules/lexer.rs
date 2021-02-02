use crate::models::token;

pub fn tokenizer(program: String) -> Result<Vec<token::Token>, &'static str> {
    let mut tokens: Vec<token::Token> = Vec::new();

    // TODO: make regex for: 
    //     - alphabets [case insensitive]
    //     - numbers
    //     - special characters 
    //     - keywords 
         
    for token in program.chars() {
        tokens.push(token::Token{
            token_type: token::TokenType::Character,
            value: token::TokenDataType::Character(token)
        });
    }
    
    Ok(tokens)
    // Err("cant lex code")
}
