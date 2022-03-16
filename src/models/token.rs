#[derive(Debug)]
#[allow(unused)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    BeginBracket,
    EndBracket,
    Operator,
    Number,
    Keyword,
    Identifier,

    /* INFO: 
     * The token types below are to be scrutinized for their use, IDENTIFER token type has been 
     * added to the enum.
     */
    SpecialChar,
    Character,
    Newline,
}

#[derive(Debug)]
#[allow(unused)]
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
