use std::fmt::Display;

use crate::token::{Token, TokenType};

pub fn lex(source_code: &str) -> Result<Vec<Token>, Vec<LexerError>> {
    let mut lexer = Lexer::new(source_code);

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let mut run = true;

    while run {
        let token = match lexer.get_next_token() {
            Ok(token) => token,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if token.ttype == TokenType::End {
            run = false;
        }

        tokens.push(token);
    }

    match errors.is_empty() {
        true => Ok(tokens),
        false => Err(errors),
    }
}

enum LexerErrorType {
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

pub struct LexerError {
    ttype: LexerErrorType,
    line_str: String,
    line: usize,
    start_pos: usize,
    end_pos: usize,
}

impl LexerError {
    fn new(ttype: LexerErrorType, lexer: &Lexer) -> Self {
        let start_of_line = lexer.start_index - lexer.start_pos;

        let mut end_of_line = lexer.index;

        while end_of_line < lexer.chars.len() && lexer.chars[end_of_line] != '\n' {
            end_of_line += 1;
        }

        Self {
            ttype,
            line_str: lexer.chars[start_of_line..end_of_line].iter().collect(),
            line: lexer.start_line,
            start_pos: lexer.start_pos,
            end_pos: lexer.pos,
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error at line {}, pos {}.\n{}", self.line, self.start_pos, self.line_str)?;

        for _ in 0..self.start_pos {
            write!(f, " ")?;
        }

        write!(f, "\x1b[1;31m")?;

        for _ in self.start_pos..self.end_pos {
            write!(f, "^")?;
        }
        
        write!(f, " {}\x1b[0m", self.ttype)
    }
}

type LexerResult<T> = Result<T, LexerError>;

struct Lexer {
    chars: Vec<char>,
    start_index: usize,
    start_line: usize,
    start_pos: usize,
    index: usize,
    line: usize,
    pos: usize,
}

impl Lexer {
    fn new(source_code: &str) -> Self {
        Self {
            chars: source_code.chars().collect(),
            start_index: 0,
            start_line: 0,
            start_pos: 0,
            index: 0,
            line: 0,
            pos: 0,
        }
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

        match self.get_next_token_value() {
            ['t', 'r', 'u', 'e'] | ['f', 'a', 'l', 's', 'e'] => self.get_token_of_type(TokenType::Bool),
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
            value: self.get_next_token_value().iter().collect(),
        }
    }

    fn get_next_token_value(&self) -> &[char] {
        match self.start_index < self.chars.len() {
            true => &self.chars[self.start_index..self.index],
            false => &['\0'],
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
