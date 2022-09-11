use std::{fmt::Display, convert::TryInto};

use crate::error_reporting::{error_reporter::Literal, scanning_err::ScanningException};

use super::scanner::Scanner;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_SQUARE,
    RIGHT_SQUARE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    COMMENT,

    // Literals.
    IDENTIFIER,
    STRING,
    INTEGER,
    FLOAT,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    LET,
    WHILE,
    CLOS,

    ERROR,

    EOF,
}

impl TokenType {
    pub fn new(c: char, next: Option<char>) -> crate::error_reporting::scanning_err::Result<(TokenType, usize)> {
        match c {
            '(' => Ok((TokenType::LEFT_PAREN, 1)),
            ')' => Ok((TokenType::RIGHT_PAREN, 1)),
            '[' => Ok((TokenType::LEFT_SQUARE, 1)),
            ']' => Ok((TokenType::RIGHT_SQUARE, 1)),
            '{' => Ok((TokenType::LEFT_BRACE, 1)),
            '}' => Ok((TokenType::RIGHT_BRACE, 1)),
            ',' => Ok((TokenType::COMMA, 1)),
            '.' => Ok((TokenType::DOT, 1)),
            '-' => Ok((TokenType::MINUS, 1)),
            '+' => Ok((TokenType::PLUS, 1)),
            ';' => Ok((TokenType::SEMICOLON, 1)),
            '*' => Ok((TokenType::STAR, 1)),
            '>' => if Self::match_char(next, '=') {return Ok((TokenType::GREATER_EQUAL, 2))} else {return Ok((TokenType::GREATER, 1))}, 
            '<' => if Self::match_char(next, '=') {return Ok((TokenType::LESS_EQUAL, 2))} else {return Ok((TokenType::LESS, 1))}, 
            '!' => if Self::match_char(next, '=') {return Ok((TokenType::BANG_EQUAL, 2))} else {return Ok((TokenType::BANG, 1))}, 
            '=' => if Self::match_char(next, '=') {return Ok((TokenType::EQUAL_EQUAL, 2))} else {return Ok((TokenType::EQUAL, 1))},
            '/' => if Self::match_char(next, '/')  {return Err(ScanningException::Commment)} else {return Ok((TokenType::SLASH, 1))},
            '\n' => Err(ScanningException::Newline),
            ' ' | '\r' | '\t' => Err(ScanningException::Ignore),
            '"' => Err(ScanningException::String),
            _ => {
                if c >= '0' && c <= '9' {
                    return Err(ScanningException::Number)
                } else {
                    Err(ScanningException::Tokenization)
                }
            },
        }
    }

    fn match_char(next: Option<char>, expected_char: char) -> bool {
        if let Some(next_uw) = next {
            return expected_char == next_uw;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Float(f32),
    Int(isize),
    String(String)
}

impl Primitive {
    fn get_value_as_str(&self) -> String {
        match self {
            Primitive::Float(float) => float.to_string(),
            Primitive::Int(int) => int.to_string(),
            Primitive::String(strng) => strng.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    tok: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<Primitive>,
}

impl Token {
    pub fn new(tok: TokenType, lexeme: String, line: usize, literal: Option<Primitive>) -> Self {
        return Self {
            tok,
            lexeme,
            line,
            literal,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = &self.literal {
            write!(f, "{} {}", self.lexeme, val.get_value_as_str())
        } else {
            write!(f, "{}", self.lexeme)
        }
    }
}

impl Literal for Token {
    fn get_line(&self) -> usize {
        return self.line;
    }
}
