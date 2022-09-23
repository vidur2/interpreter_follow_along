use std::{collections::HashSet, fmt::Display};

use crate::{
    ast::ast_types::{Binary, Expr, ExprPossibilities, Literal, Unary},
    error_reporting::{error_reporter::{ErrorReport, Unwindable}, parsing_err::ParsingException},
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

    pub fn parse(&mut self) -> Option<ExprPossibilities> {
        let expr_wrapped = self.expression();
        if let Ok(expr) = expr_wrapped {
            println!("{:?}", expr);
            return Some(expr);
        } else  if let Err(err) = expr_wrapped {
            println!("{}", err.get_value());
            return None;
        } else {
            return None;
        }
    }

    fn expression(&mut self) -> Result<ExprPossibilities, ParsingException> {
        return self.equality();
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
        if self.match_tok(&[TokenType::BANG, TokenType::MINUS]) {
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
                literal: self.previous().clone().literal.unwrap(),
            }));
        }

        if self.match_tok(&[TokenType::LEFT_PAREN]) {
            if let Ok(expr) = self.expression() {
                self.consume(TokenType::RIGHT_PAREN)?;  
                return Ok(expr);
            }
        }

        return Err(ParsingException::InvalidExpr(self.peek().clone()));
    }

    fn match_tok(&mut self, tok_types: &[TokenType]) -> bool {
        if !self.is_at_end() {
            for tok_type in tok_types.iter() {
                if self.check(tok_type) {
                    self.advance();
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
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.tokens[self.current].tok == TokenType::EOF;
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let tok = &self.previous().tok;
            if Self::multi_cmp(&[TokenType::SEMICOLON, TokenType::FUNC, TokenType::LET, TokenType::FOR, TokenType::WHILE, TokenType::IF, TokenType::PRINT, TokenType::RETURN], &tok) {
                return;
            } else {
                self.advance();
            }
        }
    }

    fn multi_cmp(tok_type: &[TokenType], tok: &TokenType) -> bool {
        for tok_cmp in tok_type.iter() {
            if tok == tok_cmp {
                return true;
            }
        }

        return false;
    }
}