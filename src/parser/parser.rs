use crate::{
    ast::expr_types::{Binary, ExprPossibilities, Literal, Ternary, Unary, Grouping, Stmt, Scope},
    error_reporting::{error_reporter::Unwindable, parsing_err::ParsingException},
    scanner::token::{ Token, TokenType },
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
        let expr_wrapped = self.func_def();
        if let Ok(expr) = expr_wrapped {
            // println!("{:?}", expr);
            return Ok(expr);
        } else if let Err(err) = expr_wrapped {
            println!("{}", err.get_value());
            return Err(ParsingException::InvalidTernaryExpr(self.peek().clone()));
        } else {
            return Err(ParsingException::PlaceHolder);
        }
    }

    fn func_def(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::FUNC]) {
            let ident = self.consume(&[TokenType::IDENTIFIER], ParsingException::PlaceHolder)?.clone();
            self.consume(&[TokenType::LEFT_PAREN], ParsingException::PlaceHolder)?;
            let mut ident_vec = Vec::new();

            while let Ok(tok) = self.consume(&[TokenType::COMMA, TokenType::IDENTIFIER], ParsingException::PlaceHolder) {
                match tok.tok {
                    TokenType::IDENTIFIER => {
                        ident_vec.push(tok.clone())
                    },
                    TokenType::COMMA => {},
                    _ => return Err(ParsingException::PlaceHolder)
                }
            }

            self.consume(&[TokenType::RIGHT_PAREN], ParsingException::PlaceHolder)?;
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::PlaceHolder)?;
            return self.scope(TokenType::FUNC, Some(ident), None, Some(ident_vec));
        }
        
        return self.while_loop();
    }

    fn while_loop(&mut self) -> Result<ExprPossibilities, ParsingException> {
        while self.match_tok(&[TokenType::WHILE]) {
            let expr = self.chain_bool()?;
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidLoop(self.previous().clone()))?;
            return self.scope(TokenType::WHILE, None, Some(Box::new(expr)), None);
        }

        return self.for_loop();
    }

    fn for_loop(&mut self) -> Result<ExprPossibilities, ParsingException> {
        while self.match_tok(&[TokenType::FOR]) {
            if !self.match_tok(&[TokenType::LEFT_BRACE]) {
                let declaration = self.declaration()?;
                return self.scope(TokenType::FOR, None, Some(Box::new(declaration)), None)
            }
        }

        return self.call_env();
    }


    fn call_env(&mut self) -> Result<ExprPossibilities, ParsingException>{
        if self.match_tok(&[TokenType::CLOSCALL]) {
            let ident = self.consume(&[TokenType::IDENTIFIER], ParsingException::InvalidEnvCall(self.previous().clone()))?.clone();
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidEnvCall(self.previous().clone()))?.clone();
            return self.scope(TokenType::CLOSCALL, Some(ident), None, None);
        }

        return self.env_dec();
    }

    fn env_dec(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::CLOS]) {
            return self.env_declaration();
        };

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
                let mut env: Scope = Scope { stmt: TokenType::CLOS, ident: Some(ident.clone()), inner: Vec::new(), condition: None, params: None };

                while !self.match_tok(&[TokenType::RIGHT_BRACE]) {
                    if self.match_tok(&[TokenType::LET]) {
                        let var = self.var_declaration(TokenType::LET)?;
                        if let ExprPossibilities::Stmt(stmt) = var && let Some(ref _expr) = stmt.inner {
                            env.inner.push(ExprPossibilities::Stmt(stmt));
                        } else {
                            return Err(ParsingException::InvalidEnv(env))
                        }
                    } else {
                        return Err(ParsingException::InvalidEnvAssign(ident))
                    }
                }
                self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
                return Ok(ExprPossibilities::Scope(env));
            }
            return Err(ParsingException::InvalidEnvAssign(ident))
        }
    }

    fn scope(&mut self, scope_type: TokenType, ident: Option<Token>, condition: Option<Box<ExprPossibilities>>, params: Option<Vec<Token>>) -> Result<ExprPossibilities, ParsingException> {
        let mut expr_list: Vec<ExprPossibilities> = Vec::new();
        while !self.match_tok(&[TokenType::RIGHT_BRACE]) {
            let expr = self.while_loop()?;
            expr_list.push(expr)
        }

        return Ok(ExprPossibilities::Scope(Scope { stmt: scope_type, ident, inner: expr_list, condition, params }));

    }

    fn declaration(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::LET]) {
            return self.var_declaration(TokenType::LET)
        };

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
                let initializer = ExprPossibilities::Stmt(Stmt { stmt: stmt_type.clone(), inner: Some(Box::new(self.ternary()?)), ident: Some(ident), params: None  });
                if self.previous().tok != TokenType::SEMICOLON && self.peek().tok != TokenType::RIGHT_PAREN {
                    self.consume(&[TokenType::NEWLINE, TokenType::SEMICOLON], ParsingException::InvalidIdentifier(self.previous().clone()))?;
                }
                return Ok(initializer);
            }
        }
        return Err(ParsingException::InvalidAssign(self.previous().clone()))
    }

    fn statement(&mut self) -> Result<ExprPossibilities, ParsingException> {
        if self.match_tok(&[TokenType::PRINT, TokenType::PRINTLN, TokenType::RETURN, TokenType::IF]) {
            if self.previous().tok == TokenType::PRINT {
                return self.print(TokenType::PRINT);
            } if self.previous().tok == TokenType::RETURN {
                let return_expr = self.expression()?;
                if self.previous().tok != TokenType::SEMICOLON {
                    self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder)?;
                }

                return Ok(ExprPossibilities::Stmt(Stmt { stmt: TokenType::RETURN, ident: None, inner: Some(Box::new(return_expr)), params: None }))
            } 
            if self.previous().tok == TokenType::IF {
                return self.if_stmt();
            } else {
                return self.print(TokenType::PRINTLN);
            }
        } 

        return self.ternary();
    }


    fn if_stmt(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let condition = self.chain_bool()?;
        self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
        let true_code = self.scope(TokenType::IF, None, Some(Box::new(condition.clone())), None)?;
        let mut false_code: Option<Box<ExprPossibilities>> = None;
        if self.match_tok(&[TokenType::ELSE]) {
            if self.peek().tok != TokenType::IF {
                self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
                let not_cond: Unary = Unary { operator: Token { tok: TokenType::BANG, lexeme: "!".to_string(), line: self.peek().line, literal: None }, right: Box::new(condition.clone()) } ;
                false_code = Some(Box::new(self.scope(TokenType::IF, None, Some(Box::new(ExprPossibilities::Unary(not_cond))), None)?));
            } else {
                false_code = Some(Box::new(self.statement()?));
            }
        }

        return Ok(ExprPossibilities::Ternary(Ternary { condition: Box::new(condition), false_cond: false_code, true_cond: Some(Box::new(true_code)) }))
    }

    fn print(&mut self, tok: TokenType) -> Result<ExprPossibilities, ParsingException> {
        let expr = self.ternary()?;
        if let ExprPossibilities::Grouping(expr) = expr {
            self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
            return Ok(ExprPossibilities::Stmt(Stmt { stmt: tok, inner: Some(Box::new(ExprPossibilities::Grouping(expr))), ident: None, params: None }));
        } else {
            return Err(ParsingException::InvalidPrint(self.previous().clone()));
        }
    } 

    fn ternary(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let expr = self.chain_bool()?;
        if self.match_tok(&[TokenType::TERNARYTRUE]) {
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
            let true_case = self.scope(TokenType::IF, None, None, None)?;

            if self.match_tok(&[TokenType::TERNARYFALSE]) {
                self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
                let false_case = self.scope(TokenType::IF, None, None, None)?;
                self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: Some(Box::new(true_case)),
                }));
            } else {
                self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: None,
                    true_cond: Some(Box::new(true_case)),
                }));
            }
        } else if self.match_tok(&[TokenType::TERNARYFALSE]) {
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
            let false_case = self.scope(TokenType::IF, None, None, None)?;

            if self.match_tok(&[TokenType::TERNARYTRUE]) {
                self.consume(&[TokenType::LEFT_BRACE], ParsingException::InvalidTernaryExpr(self.previous().clone()))?;
                let true_case = self.scope(TokenType::IF, None, None, None)?;
                self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: Some(Box::new(true_case)),
                }));
            } else {
                self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
                return Ok(ExprPossibilities::Ternary(Ternary {
                    condition: Box::new(expr),
                    false_cond: Some(Box::new(false_case)),
                    true_cond: None,
                }));
            }
        } else {
            // self.consume(&[TokenType::LEFT_BRACE], ParsingException::PlaceHolder);
            return Ok(expr);
        }
    }

    fn chain_bool(&mut self) -> Result<ExprPossibilities, ParsingException> {
        let mut expr = self.expression()?;

        while self.match_tok(&[TokenType::AND, TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.expression()?;
            expr = ExprPossibilities::Binary(Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            })
        }

        return Ok(expr);
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
            });
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
            });
            self.consume(&[TokenType::SEMICOLON], ParsingException::PlaceHolder);
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
            let ident = self.previous().clone();
            if self.match_tok(&[TokenType::LEFT_PAREN]) {
                let mut arg_vec: Vec<ExprPossibilities> = Vec::new();
                while !self.match_tok(&[TokenType::RIGHT_PAREN]) {
                    let curr = self.expression()?;
                    arg_vec.push(curr);

                    let res = self.consume(&[TokenType::COMMA], ParsingException::PlaceHolder);

                    if let Err(err) = res && TokenType::RIGHT_PAREN != self.peek().tok {
                        return Err(err);
                    } 
                }

                self.match_tok(&[TokenType::SEMICOLON]);
                return Ok(ExprPossibilities::Stmt( Stmt { stmt: TokenType::FUNC, ident: Some(ident), inner: None, params: Some(Box::new(arg_vec)) }))
            }
            return Ok(ExprPossibilities::Stmt(Stmt { stmt: TokenType::IDENTIFIER, ident: Some(ident), inner: None , params: None}))
        }

        if self.match_tok(&[TokenType::LEFT_PAREN]) {
            let mut inner = Vec::new();

            while let Ok(expr) = self.declaration() {
                inner.push(expr);
            }
            self.consume(&[TokenType::RIGHT_PAREN], ParsingException::UnterminatedParenthesis(self.previous().clone()))?;
            self.consume(&[TokenType::LEFT_BRACE], ParsingException::PlaceHolder);
            return Ok(ExprPossibilities::Grouping(Grouping {
                expr: Box::new(inner),
            }));
        }

        if self.match_tok(&[TokenType::SEMICOLON]) {
            return self.primary();
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
