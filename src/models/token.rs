#[derive(Debug)]
pub enum TokenType {
    OpenParenthesis,
    CloseParenthesis,
    Operator,
    Character,
    Number,
    SpecialChar,
    Keyword,
}

#[derive(Debug)]
pub enum TokenDataType {
    Word(String),
    Character(char),
    Number(i32),
    Keyword(String),
}

pub struct Token {
    pub token_type: TokenType,
    pub value: TokenDataType,
}
