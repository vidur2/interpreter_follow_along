use crate::{
    ast::expr_types::{Binary, ExprPossibilities, Literal, Ternary, Unary, Grouping, Stmt, EnvParsable},
    error_reporting::{error_reporter::Unwindable, parsing_err::ParsingException},
    scanner::token::{Token, TokenType, Primitive},
};

#[derive(Clone)]
pub struct Parser {
    pub current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { current: 0, tokens };
    }

    pub fn parse(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let expr_wrapped = self.env_dec();
        if let Ok(expr) = expr_wrapped {
            return Ok(expr);
        } else if let Err(err) = expr_wrapped {
            println!("{}", err.get_value());
            return Err(ParsingException::InvalidTernaryExpr(self.peek().clone()));
        } else {
            return Err(ParsingException::PlaceHolder);
        }
    }

    fn env_dec(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::CLOS]) {return self.env_declaration();};

        return self.declaration();
    }

    fn env_declaration(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let name = self.consume(&[TokenType::IDENTIFIER], ParsingException::PlaceHolder);

        if let Err(err) = name  {
            return Err(err)
        }

        unsafe {
            let ident = name.unwrap_unchecked().clone();
            if self.match_tok(&[TokenType::EQUAL]) && self.match_tok(&[TokenType::LEFT_BRACE]) {
                let mut env: EnvParsable = EnvParsable { stmt: TokenType::CLOS, ident: ident.clone(), inner: Vec::new() };

                while self.peek().tok == TokenType::LET {
                    let var = self.var_declaration(TokenType::LET)?;

                    if let ExprPossibilities::Stmt(stmt) = var && let Some(ref _expr) = stmt.inner {
                        env.inner.push(stmt);
                    } else {
                        return Err(ParsingException::InvalidEnv(env))
                    }

                }

                return Ok(ExprPossibilities::Env(env));
            }
            return Err(ParsingException::InvalidEnvAssign(ident))
        }
    }

    fn declaration(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::LET]) {return self.var_declaration(TokenType::LET)};

        return self.statement();
    }

    fn var_declaration(&mut self, stmt_type: TokenType) -> Result<ExprPossibilities, ParsingException> {
        let name = self.consume(&[TokenType::IDENTIFIER], ParsingException::PlaceHolder);
        if let Err(err) = name {
            return Err(err);
        }
        unsafe {
            let ident = name.unwrap_unchecked().clone();
            if self.match_tok(&[TokenType::EQUAL]) {
                let initializer = ExprPossibilities::Stmt(Stmt { stmt: stmt_type, inner: Some(Box::new(self.ternary()?)), ident: Some(ident)  });
                self.consume(&[TokenType::NEWLINE, TokenType::SEMICOLON], ParsingException::InvalidIdentifier(self.previous().clone()))?;
                return Ok(initializer);
            }
        }
        return Err(ParsingException::InvalidAssign(self.previous().clone()))
    }

    fn statement(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::PRINT, TokenType::PRINTLN]) {
            if self.previous().tok == TokenType::PRINT {
                return self.print(TokenType::PRINT);
            } else {
                return self.print(TokenType::PRINTLN);
            }
        } 

        return self.ternary();
    }

    fn print(&mut self, tok: TokenType) -> Result<ExprPossibilities, ParsingException> {
        let expr = self.ternary()?;
        if let ExprPossibilities::Grouping(expr) = expr {
            return Ok(ExprPossibilities::Stmt(Stmt { stmt: tok, inner: Some(Box::new(ExprPossibilities::Grouping(expr))), ident: None }));
        } else {
            return Err(ParsingException::InvalidPrint(self.previous().clone()));
        }
    } 

    fn ternary(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let expr = self.expression()?;
        if self.match_tok(&[TokenType::TERNARYTRUE]) {
            let true_case = self.expression()?;

            if self.match_tok(&[TokenType::TERNARYFALSE]) {
                let false_case = self.expression()?;
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: Some(Box::new(true_case)),
                }));
            } else {
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: None,
                    true_cond: Some(Box::new(true_case)),
                }));
            }
        } else if self.match_tok(&[TokenType::TERNARYFALSE]) {
            let false_case = self.expression()?;

            if self.match_tok(&[TokenType::TERNARYTRUE]) {
                let true_case = self.expression()?;
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: Some(Box::new(true_case)),
                }));
            } else {
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: None,
                }));
            }
        } else {
            return Ok(expr);
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

            expr = ExprPossibilities::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            })
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.unary()?;

        while self.match_tok(&[TokenType::SLASH, TokenType::STAR, TokenType::MODULO]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = ExprPossibilities::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            })
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(ExprPossibilities::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
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

        if self.match_tok(&[TokenType::IDENTIFIER]) {
            return Ok(ExprPossibilities::Stmt(Stmt { stmt: TokenType::IDENTIFIER, ident: Some(self.previous().clone()), inner: None }))
        }

        if self.match_tok(&[TokenType::LEFT_PAREN]) {
            if let Ok(expr) = self.expression() {
                self.consume(&[TokenType::RIGHT_PAREN], ParsingException::UnterminatedParenthesis(self.previous().clone()))?;
                return Ok(ExprPossibilities::Grouping(Grouping {
                    expr: Box::new(expr),
                }));
            }
        }

        if self.current > 0 {
            return Err(ParsingException::InvalidExpr(self.previous().clone()));
        } else {
            return Err(ParsingException::InvalidExpr(self.peek().clone()))
        }
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

    pub fn is_at_end(&self) -> bool {
        return self.current > self.tokens.len() - 1|| self.tokens[self.current].tok == TokenType::EOF;
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn consume(&mut self, tok_type: &[TokenType], exception: ParsingException) -> Result<&Token, ParsingException> {
        for indiv_tok in tok_type.iter() {
            if self.check(indiv_tok) {
                return Ok(&self.advance());
            };
        }
        return Err(exception);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let tok = &self.previous().tok;
            if Self::multi_cmp(
                &[
                    TokenType::SEMICOLON,
                    TokenType::FUNC,
                    TokenType::LET,
                    TokenType::FOR,
                    TokenType::WHILE,
                    TokenType::IF,
                    TokenType::PRINT,
                    TokenType::RETURN,
                ],
                &tok,
            ) {
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
