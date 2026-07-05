use std::{collections::HashMap, fmt::Display};

use crate::{mon_error::MonError, mon_object::MonObject, token::{Token, TokenType}};

pub fn parse(tokens: Vec<Token>) -> ParserResult<MonObject> {
    let mut parser = Parser::new(tokens);

    let value = parser.value()?;

    match parser.index >= parser.tokens.len() {
        true => Ok(value),
        false => Err(ParserError::new(ParserErrorType::UnexpectedToken(parser.cur().ttype), &parser.cur().text)),
    }
}

pub enum ParserErrorType {
    UnexpectedToken(TokenType),
    ExpectedEnum,
    InvalidExpression,
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorType::UnexpectedToken(expected) => write!(f, "expected {}", expected.get_name()),
            ParserErrorType::ExpectedEnum => write!(f, "expected {} or {}", TokenType::LeftBracket.get_name(), TokenType::LeftBrace.get_name()),
            ParserErrorType::InvalidExpression => write!(f, "expected value"),
        }
    }
}

type ParserError = MonError<ParserErrorType>;
type ParserResult<T> = Result<T, ParserError>;

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    fn value(&mut self) -> ParserResult<MonObject> {
        Ok(match self.cur().ttype {
            TokenType::Number => MonObject::Number(self.advance().text.to_string()),
            TokenType::Bool => MonObject::Bool(self.advance().text.to_string()),
            TokenType::String => MonObject::String(self.advance().text.to_string()),
            TokenType::Identifier => {
                // This helps because it allows early returns
                let mut f = || {
                    let name = self.advance().text.to_string();

                    if let Ok(object) = self.array() {
                        return Ok(MonObject::Enum(name, Box::new(object)))
                    }

                    if let Ok(object) = self.dictionary() {
                        return Ok(MonObject::Enum(name, Box::new(object)))
                    }

                    Err(ParserError::new(ParserErrorType::ExpectedEnum, &self.cur().text))
                };

                f()?
            }
            TokenType::LeftBracket => self.array()?,
            TokenType::LeftBrace => self.dictionary()?,
            _ => return Err(ParserError::new(ParserErrorType::InvalidExpression, &self.cur().text)),
        })
    }

    fn array(&mut self) -> ParserResult<MonObject> {
        self.consume(TokenType::LeftBracket)?;

        let mut values = Vec::new();

        while self.cur().ttype != TokenType::RightBracket {
            values.push(self.value()?);

            if self.consume(TokenType::Comma).is_err() {
                break;
            }
        }

        self.consume(TokenType::RightBracket)?;

        Ok(MonObject::Array(values))
    }

    fn dictionary(&mut self) -> ParserResult<MonObject> {
        self.consume(TokenType::LeftBrace)?;

        let mut values = HashMap::new();

        while self.cur().ttype != TokenType::RightBrace {
            self.consume(TokenType::Identifier)?;

            let key = self.prev().text.to_string();

            self.consume(TokenType::Equals)?;

            values.insert(key, self.value()?);

            if self.consume(TokenType::Comma).is_err() {
                break;
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(MonObject::Dictionary(values))
    }

    fn cur(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn prev(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    fn advance(&mut self) -> &Token {
        self.index += 1;
        self.prev()
    }

    fn consume(&mut self, ttype: TokenType) -> ParserResult<()> {
        match self.cur().ttype == ttype {
            true => self.advance(),
            false => return Err(ParserError::new(ParserErrorType::UnexpectedToken(ttype), &self.cur().text)),
        };

        Ok(())
    }
}
