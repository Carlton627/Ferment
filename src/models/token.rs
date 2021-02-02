#[derive(Debug)]
pub enum TokenType {
    OpenParenthesis,
    CloseParenthesis,
    Character,
    Number,
    SpecialChar,
    Keyword,
}

#[derive(Debug)]
pub enum TokenDataType {
    Character(char),
    Number(i32),
    Keyword(String),
}

pub struct Token {
    pub token_type: TokenType,
    pub value: TokenDataType,
}
