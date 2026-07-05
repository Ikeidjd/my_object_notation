use std::{fmt::Display, ops::{Deref, DerefMut}, rc::Rc};

use crate::{mon_error::MonError, text::Text, token::{Token, TokenType}};

pub fn lex(source_code: Rc<[char]>) -> Result<Vec<Token>, Vec<LexerError>> {
    let mut lexer = Lexer::new(source_code);

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    loop {
        let token = match lexer.get_next_token() {
            Ok(token) => token,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if token.ttype == TokenType::End {
            break;
        }

        tokens.push(token);
    }

    match errors.is_empty() {
        true => Ok(tokens),
        false => Err(errors),
    }
}

pub enum LexerErrorType {
    InvalidCharacter,
    UnclosedStringLiteral,
}

impl Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LexerErrorType::InvalidCharacter => "invalid character",
            LexerErrorType::UnclosedStringLiteral => "unclosed string literal",
        })
    }
}

type LexerError = MonError<LexerErrorType>;
type LexerResult<T> = Result<T, LexerError>;

struct Lexer(Text);

impl Lexer {
    fn new(chars: Rc<[char]>) -> Self {
        Self(Text {
            chars,
            start_index: 0,
            start_line: 0,
            start_pos: 0,
            index: 0,
            line: 0,
            pos: 0,
        })
    }

    fn get_next_token(&mut self) -> LexerResult<Token> {
        self.skip_whitespace();

        self.start_index = self.index;
        self.start_line = self.line;
        self.start_pos = self.pos;

        let c = self.advance();

        if c.is_ascii_digit() {
            return Ok(self.number());
        }

        if Self::is_identifier_char(c) {
            return Ok(self.identifier_or_bool());
        }

        if c == '"' {
            return self.string();
        }

        let ttype = match c {
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '=' => TokenType::Equals,
            ',' => TokenType::Comma,
            '\0' => TokenType::End,
            _ => return Err(LexerError::new(LexerErrorType::InvalidCharacter, self)),
        };

        Ok(self.get_token_of_type(ttype))
    }

    fn number(&mut self) -> Token {
        while self.cur().is_numeric() {
            self.advance();
        }

        self.get_token_of_type(TokenType::Number)
    }

    fn identifier_or_bool(&mut self) -> Token {
        while Self::is_identifier_char(self.cur()) {
            self.advance();
        }

        match &self.0.to_string()[..] {
            "true" | "false" => self.get_token_of_type(TokenType::Bool),
            _ => self.get_token_of_type(TokenType::Identifier),
        }
    }

    fn string(&mut self) -> LexerResult<Token> {
        while self.cur() != '"' && self.cur() != '\n' && self.cur() != '\0' {
            self.advance();
        }

        if self.cur() != '"' {
            return Err(LexerError::new(LexerErrorType::UnclosedStringLiteral, self));
        }

        self.advance();

        Ok(self.get_token_of_type(TokenType::String))
    }

    fn get_token_of_type(&mut self, ttype: TokenType) -> Token {
        Token {
            ttype,
            text: self.0.clone(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur().is_ascii_whitespace() || self.cur() == '#' {
            if self.advance() != '#' {
                continue;
            }

            loop {
                let c = self.advance();

                if c == '\n' || c == '\0' {
                    break;
                }
            }
        }
    }

    fn cur(&self) -> char {
        match self.index < self.chars.len() {
            true => self.chars[self.index],
            false => '\0',
        }
    }

    fn advance(&mut self) -> char {
        let c = self.cur();
        self.index += 1;

        match c {
            '\n' => {
                self.line += 1;
                self.pos = 0;
            },
            _ => self.pos += 1,
        }

        c
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
}

impl Deref for Lexer {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
