use crate::{
    error_reporting::{
        error_reporter::{ErrorReport, Literal, Unwindable},
        scanning_err::ScanningException,
    },
    parser::parser::Parser,
};

use super::token::{Primitive, Token, TokenType};

use std::{fmt::Display, io::Write};

#[derive(Clone)]
pub struct Scanner {
    buff: String,
    token: Vec<Token>,
    has_error: bool,
    curr_line: usize,
    start: usize,
    curr_char: usize,
    error: Vec<(ScanningException, Token)>,
}

impl ErrorReport for Scanner {
    fn print_error<E: Unwindable, T: Display + Literal + Clone>(error: E, literal: Option<T>) {
        println!(
            "{} on token: {:#} on line {}",
            error.get_value(),
            literal.clone().unwrap(),
            literal.unwrap().get_line()
        );
    }
}

impl Scanner {
    // File Input Mode
    pub fn input_file<'b>(path: &'b str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path);

        match contents {
            Ok(file) => {
                return Ok(Self {
                    buff: file,
                    token: Vec::new(),
                    has_error: false,
                    curr_line: 1,
                    start: 0,
                    curr_char: 0,
                    error: Vec::new(),
                })
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub fn get_buff(&self) -> Vec<Token> {
        return self.token.clone();
    }

    fn start_scanner() -> Self {
        return Self {
            buff: String::new(),
            token: Vec::new(),
            has_error: false,
            curr_line: 1,
            curr_char: 0,
            start: 0,
            error: Vec::new(),
        };
    }

    // Shell mode
    pub fn accept_input() {
        let mut lexer = Self::start_scanner();

        loop {
            print!("-> ");
            std::io::stdout().flush().unwrap();
            let mut line: String = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut line) {
                if line.contains("exit()") {
                    break;
                } else {
                    lexer.token.pop();
                    lexer.read_as_buff(line);
                    lexer.tokenize_buff();
                    // println!("{:?}", lexer.token);
                    lexer.token.push(Token {
                        tok: TokenType::EOF,
                        lexeme: String::new(),
                        line: usize::MAX,
                        literal: None,
                    });

                    let mut parser = Parser::new(lexer.token.clone());

                    while !parser.is_at_end() {
                        let expr = parser.parse();
                        if let Err(err) = expr {
                            parser.current += 1;
                        }
                    }
                    lexer.curr_char = 0;
                    lexer.start = 0;
                }
            } else {
                println!("Invalid line format")
            }
        }
    }

    fn read_as_buff(&mut self, data: String) {
        self.buff = data;
    }

    pub fn tokenize_buff(&mut self) {
        let buff_len_idx = self.buff.len() - 1;
        let cloned_ref = self.clone();
        let bytes = cloned_ref.buff.as_bytes();
    

        while !self.is_at_end() {
            let c: char = bytes[self.curr_char] as char;
            let mut next: Option<char> = None;
            if self.curr_char < buff_len_idx {
                next = Some(self.buff.as_bytes()[self.curr_char + 1] as char);
            }
            let tok_type = TokenType::new(c, next);
            self.match_tok(tok_type, bytes);
            self.start = self.curr_char;
        }

        for (err, tok) in self.error.drain(0..) {
            Self::print_error(err, Some(tok));
        }
    }

    fn match_tok(&mut self, tok_type: Result<(TokenType, usize), ScanningException>, buff: &[u8]) {
        if let Ok(tok_type_uw) = tok_type {
            self.add_token(tok_type_uw.0, None);
            self.advance_by(tok_type_uw.1);
        } else if let Err(ScanningException::Newline) = tok_type {
            self.advance_by(1);
            self.curr_line += 1;
        } else if let Err(ScanningException::Commment) = tok_type {
            self.advance_line();
        } else if let Err(ScanningException::Tokenization) = tok_type {
            // self.has_error = true;
            // self.advance_by(1);
            // self.error.push((ScanningException::Tokenization, Token::new(TokenType::ERROR, self.buff[self.start..self.curr_char].to_string(), self.curr_line, None)));
            self.identifier();
            self.advance_by(1);
        } else if let Err(ScanningException::Number) = tok_type {
            self.handle_num(buff);
            self.advance_by(1);
        } else if let Err(ScanningException::String) = tok_type {
            if let Some(err) = self.handle_str(buff) {
                self.error.push((
                    err,
                    Token::new(
                        TokenType::ERROR,
                        self.buff[self.start..self.curr_char].to_string(),
                        self.curr_line,
                        None,
                    ),
                ));
            }
            self.advance_by(1)
        } else if let Err(ScanningException::Ignore) = tok_type {
            self.advance_by(1);
        }
    }

    fn handle_str(&mut self, buff: &[u8]) -> Option<ScanningException> {
        let mut literal_val = String::new();
        loop {
            self.advance_by(1);
            let curr_char = buff[self.curr_char] as char;
            if curr_char == '"' {
                self.add_token(TokenType::STRING, Some(Primitive::String(literal_val)));
                return None;
            } else if curr_char == '\n' || curr_char == ';' {
                self.advance_by(1);
                self.curr_line += 1;
                self.has_error = true;
                return Some(ScanningException::UnterminatedString);
            }

            literal_val.push(curr_char);
        }
    }

    fn handle_num(&mut self, buff: &[u8]) {
        let mut num_val: isize = 0;
        let mut float_val: f32 = 0.0;
        let mut has_decimal = false;
        let mut digit_count = 0;

        loop {
            let curr_char = buff[self.curr_char] as char;
            if curr_char == '.' && !has_decimal {
                let next_char = buff[self.curr_char] as char;
                if next_char >= '0' && next_char <= '9' {
                    has_decimal = true;
                    digit_count = 1;
                    float_val = num_val as f32;
                }
            } else if !(curr_char <= '9' && curr_char >= '0') {
                if curr_char == '\n' {
                    self.curr_line += 1;
                }

                if has_decimal {
                    self.add_token(TokenType::FLOAT, Some(Primitive::Float(float_val)));
                } else {
                    self.add_token(TokenType::INTEGER, Some(Primitive::Int(num_val)))
                }
                self.curr_char -= 1;
                break;
            } else if curr_char <= '9' && curr_char >= '0' && has_decimal {
                float_val +=
                    curr_char.to_digit(10).unwrap() as f32 / (10f32.powf(digit_count as f32));
                digit_count += 1;
            } else if curr_char <= '9' && curr_char >= '0' {
                num_val = num_val * 10 + curr_char.to_digit(10).unwrap() as isize;
            }

            self.advance_by(1);
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance_by(1);
        }

        let substr = self.buff.get(self.start..self.curr_char).unwrap();

        let tok_type = TokenType::match_keyword(substr);

        self.add_token(tok_type, None);
    }

    fn advance_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance_by(1)
        }
        self.curr_line += 1;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.buff.as_bytes()[self.curr_char] as char;
        }
    }

    fn is_at_end(&self) -> bool {
        return self.curr_char >= self.buff.len();
    }

    fn add_token(&mut self, tok: TokenType, literal: Option<Primitive>) {
        let lexeme = &self.buff[self.start..self.curr_char];
        self.token.push(Token::new(
            tok,
            String::from(lexeme),
            self.curr_line,
            literal,
        ));
    }

    fn is_alpha(c: char) -> bool {
        return c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z';
    }

    fn is_alpha_numeric(c: char) -> bool {
        return Self::is_alpha(c) || c >= '0' && c <= '9' || c == '&' || c == '|';
    }

    fn advance_by(&mut self, num: usize) {
        self.curr_char += num;
    }
}
