use crate::text::Text;

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub text: Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    PrimitiveTypeLiteral,
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

impl TokenType {
    pub fn get_name(&self) -> &str {
        match self {
            TokenType::PrimitiveTypeLiteral => "primitive type literal",
            TokenType::String => "string",
            TokenType::Identifier => "identifier",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Equals => "=",
            TokenType::Comma => ",",
            TokenType::End => "end",
        }
    }
}
