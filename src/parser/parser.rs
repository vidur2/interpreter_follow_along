use std::collections::HashSet;

use crate::{
    ast::ast_types::{Binary, Expr, ExprPossibilities, Literal, Unary},
    error_reporting::{error_reporter::ErrorReport, parsing_err::ParsingException},
    scanner::token::{Token, TokenType},
};

#[derive(Clone)]
pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { current: 0, tokens };
    }

    fn expression(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let eq = self.equality();
        match eq {
            Ok(expr) => return Ok(expr),
            Err(e) => {
                // Self::print_error(e, None);

                return Err(e);
            },
        }
    }

    fn equality(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.comparison()?;

        while self.match_tok(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = ExprPossibilities::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            })
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.term()?;

        while self.match_tok(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = ExprPossibilities::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            })
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.factor()?;

        while self.match_tok(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = ExprPossibilities::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator })
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.unary()?;

        while self.match_tok(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = ExprPossibilities::Binary(Binary { left: Box::new(expr), right: Box::new(right), operator })
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if (self.match_tok(&[TokenType::BANG, TokenType::MINUS])) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(ExprPossibilities::Unary(Unary { operator, right: Box::new(right) }))
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::FALSE]) {
            return Ok(ExprPossibilities::Literal(Literal {
                literal: crate::scanner::token::Primitive::Bool(false),
            }));
        };
        if self.match_tok(&[TokenType::TRUE]) {
            return Ok(ExprPossibilities::Literal(Literal {
                literal: crate::scanner::token::Primitive::Bool(true),
            }));
        };
        if self.match_tok(&[TokenType::NIL]) {
            return Ok(ExprPossibilities::Literal(Literal {
                literal: crate::scanner::token::Primitive::None,
            }));
        };

        if self.match_tok(&[TokenType::INTEGER, TokenType::STRING, TokenType::FLOAT]) {
            return Ok(ExprPossibilities::Literal(Literal {
                literal: self.peek().clone().literal.unwrap(),
            }));
        }

        if self.match_tok(&[TokenType::LEFT_PAREN]) {
            if let Ok(expr) = self.expression() {
                self.consume(TokenType::RIGHT_PAREN)?;  
                return Ok(expr);
            }
        }

        todo!();
    }

    fn match_tok(&self, tok_types: &[TokenType]) -> bool {
        if !self.is_at_end() {
            for tok_type in tok_types.iter() {
                if self.check(tok_type) {
                    return true;
                }
            }
        }

        return false;
    }

    fn check(&self, tok_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        return &self.peek().tok == tok_type;
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.current == self.tokens.len() - 1;
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn consume(
        &mut self,
        tok_type: TokenType,
    ) -> Result<&Token, ParsingException> {
        if self.check(&tok_type) {
            return Ok(&self.advance());
        };

        return Err(ParsingException::UnterminatedParenthesis(
            self.peek().clone(),
        ));
    }
}

impl ErrorReport for Parser {
    fn print_error<
        E: crate::error_reporting::error_reporter::Unwindable,
        T: std::fmt::Display + crate::error_reporting::error_reporter::Literal,
    >(
        error: E,
        literal: Option<T>,
    ) {
        println!("{}", error.get_value())
    }
}
