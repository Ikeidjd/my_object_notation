#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Bool,
    String,
    Identifier,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equals,
    Comma,
    End,
}
