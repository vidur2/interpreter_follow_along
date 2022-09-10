use crate::error_reporting::{error_reporter::{ErrorReport, Literal, Unwindable}, scanning_err::ScanningError};

use super::token::{Token, TokenType};
use std::{
    collections::LinkedList,
    fmt::{Debug, Display},
    io::Write,
};

pub struct Scanner {
    buff: String,
    token: Vec<Token>,
    has_error: bool,
    curr_line: usize,
    start: usize,
    curr_char: usize,
    error: Vec<(ScanningError, Token)>
}

impl ErrorReport for Scanner {
    fn print_error<E: Unwindable, T: Display + Literal>(error: E, literal: T) {
        println!(
            "{} on token: {:#} on line {}",
            error.get_value(),
            literal,
            literal.get_line()
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
                    error: Vec::new()
                })
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn start_scanner() -> Self {
        return Self {
            buff: String::new(),
            token: Vec::new(),
            has_error: false,
            curr_line: 1,
            curr_char: 0,
            start: 0,
            error: Vec::new()
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
                lexer.read_as_buff(line);
                lexer.tokenize_buff();
                // println!("{:?}", lexer.token);
                lexer.curr_char = 0;
                lexer.start = 0;
            } else {
                println!("Invalid line format")
            }
        }
    }

    fn read_as_buff(&mut self, data: String) {
        self.buff = data;
    }

    fn tokenize_buff(&mut self) {
        let buff_len_idx = self.buff.len() - 1;

        while !self.is_at_end() {
            let c: char = self.buff.as_bytes()[self.curr_char] as char;
            let mut next: Option<char> = None; 
            if self.curr_char < buff_len_idx {
                next = Some(self.buff.as_bytes()[self.curr_char + 1] as char);
            }
            let tok_type = TokenType::new(c, next);
            if let Ok(tok_type_uw) = tok_type {
                self.add_token(tok_type_uw.0, None);
                self.advance_by(tok_type_uw.1);
            } else if let Err(ScanningError::Newline) = tok_type {
                self.advance_by(1);
                self.curr_line += 1;
            } else if let Err(ScanningError::Tokenization) = tok_type {
                self.has_error = true;
                self.advance_by(1);
                self.error.push((ScanningError::Tokenization, Token::new(TokenType::ERROR, self.buff[self.start..self.curr_char].to_string(), self.curr_line, None)));
            }
            self.start = self.curr_char;
        }


        for (err, tok) in self.error.drain(0..) {
            Self::print_error(err, tok);
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' } else {return self.buff.as_bytes()[self.curr_char] as char}
    }

    fn is_at_end(&self) -> bool {
        return self.curr_char >= self.buff.len();
    }

    fn add_token(&mut self, tok: TokenType, literal: Option<String>) {
        let lexeme = &self.buff[self.start..self.curr_char];
        self.token.push(Token::new(
            tok,
            String::from(lexeme),
            self.curr_line,
            literal,
        ));
    }

    fn advance_by(&mut self, num: usize) {
        self.curr_char += num;
    }
}
