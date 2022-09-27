use std::{convert::TryInto, fmt::Display, ops::Add};

use crate::error_reporting::{error_reporter::Literal, scanning_err::ScanningException};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
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
    MODULO,
    TERNARYTRUE,
    TERNARYFALSE,
    NEWLINE,

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
    ELSE,
    FALSE,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    PRINTLN,
    RETURN,
    SELF,
    TRUE,
    LET,
    WHILE,
    CLOS,
    SWITCH,
    FUNC,

    ERROR,

    EOF,
}

impl TokenType {
    pub fn new(
        c: char,
        next: Option<char>,
    ) -> crate::error_reporting::scanning_err::Result<(TokenType, usize)> {
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
            '%' => Ok((TokenType::MODULO, 1)),
            '?' => {Ok((TokenType::TERNARYTRUE, 1))},
            ':' => Ok((TokenType::TERNARYFALSE, 1)),
            '>' => {
                if Self::match_char(next, '=') {
                    return Ok((TokenType::GREATER_EQUAL, 2));
                } else {
                    return Ok((TokenType::GREATER, 1));
                }
            }
            '<' => {
                if Self::match_char(next, '=') {
                    return Ok((TokenType::LESS_EQUAL, 2));
                } else {
                    return Ok((TokenType::LESS, 1));
                }
            }
            '!' => {
                if Self::match_char(next, '=') {
                    return Ok((TokenType::BANG_EQUAL, 2));
                } else {
                    return Ok((TokenType::BANG, 1));
                }
            }
            '=' => {
                if Self::match_char(next, '=') {
                    return Ok((TokenType::EQUAL_EQUAL, 2));
                } else {
                    return Ok((TokenType::EQUAL, 1));
                }
            }
            '/' => {
                if Self::match_char(next, '/') {
                    return Err(ScanningException::Commment);
                } else {
                    return Ok((TokenType::SLASH, 1));
                }
            }
            '\n' => Err(ScanningException::Newline),
            ' ' | '\r' | '\t' => Err(ScanningException::Ignore),
            '"' => Err(ScanningException::String),
            _ => {
                if c >= '0' && c <= '9' {
                    return Err(ScanningException::Number);
                } else {
                    Err(ScanningException::Tokenization)
                }
            }
        }
    }

    pub fn match_keyword(s: &str) -> Self {
        match s {
            "and" | "&&" => TokenType::AND,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "if" => TokenType::IF,
            "null" => TokenType::NIL,
            "or" | "||" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "self" => TokenType::SELF,
            "true" => TokenType::TRUE,
            "let" => TokenType::LET,
            "while" => TokenType::WHILE,
            "env" => TokenType::CLOS,
            "switch" => TokenType::SWITCH,
            "func" => TokenType::FUNC,
            "println" => TokenType::PRINTLN,
            _ => TokenType::IDENTIFIER,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Float(f32),
    Int(isize),
    String(String),
    Bool(bool),
    None,
}

impl Primitive {
    pub fn get_value_as_str(&self) -> Option<String> {
        match self {
            Primitive::Float(float) => Some(float.to_string()),
            Primitive::Int(int) => Some(int.to_string()),
            Primitive::String(strng) => Some(strng.to_string()),
            Primitive::Bool(boolean) => Some(boolean.to_string()),
            Primitive::None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Primitive>,
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
        if let Some(val) = &self.literal && let Some(val_uw) = val.get_value_as_str(){
            write!(f, "{} {}", self.lexeme, val_uw)
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
