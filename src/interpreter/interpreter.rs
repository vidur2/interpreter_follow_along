use std::{rc::Rc, collections::HashMap};

use crate::{ast::{ast_traits::{Interperable, Accept}, expr_types::{ExprPossibilities}}, scanner::token::{Primitive, TokenType, Func}, error_reporting::{interp_err::InterpException, parsing_err::ParsingException, error_reporter::Unwindable}};

use super::environment::Environment;

pub struct Interpreter {
    pub globals: Environment,
}


impl Interpreter {
    pub fn new() -> Self {
        return Self {
            globals: Environment::new()
        }
    }
    
    pub fn interpret(&mut self, expr: &ExprPossibilities) {
        let eval = self.evaluate(expr);
        if let Ok(prim) = eval {
            // match prim {
            //     Primitive::Float(flt) => println!("{}", flt),
            //     Primitive::Int(int) => println!("{}", int),
            //     Primitive::String(string) => println!("{}", string),
            //     Primitive::Bool(boolean) => println!("{}", boolean),
            //     Primitive::None => print!(""),
            // }
        } else if let Err(err) = eval {
            println!("{}", err.get_value())
        }
    } 

    fn evaluate(&mut self, expr: &ExprPossibilities) -> Result<Primitive, InterpException> {
        return ExprPossibilities::accept(expr.clone(), self);
    }

    fn convert_bool(b: bool) -> isize {
        if b {
            return 1;
        } else {
            return 0;
        }
    }
}

impl Interperable<Result<Primitive, InterpException>> for Interpreter {
    fn visit_expr(&mut self, expr: crate::ast::expr_types::ExprPossibilities) -> Result<Primitive, InterpException> {
        match expr {
            crate::ast::expr_types::ExprPossibilities::Binary(bin) => {
                let left = self.evaluate(&bin.left)?;
                let right = self.evaluate(&bin.right)?;
                match bin.operator.tok {
                    TokenType::AND => {
                        if let Primitive::Bool(bool1) = left && let Primitive::Bool(bool2) = right {
                            return Ok(Primitive::Bool(bool1 && bool2));
                        } else {
                            return Err(InterpException::PlaceHolder);
                        }
                    },
                    TokenType::OR => {
                        if let Primitive::Bool(bool1) = left && let Primitive::Bool(bool2) = right {
                            return Ok(Primitive::Bool(bool1 || bool2));
                        } else {
                            return Err(InterpException::PlaceHolder);
                        }
                    }
                    TokenType::SLASH => {
                        if Primitive::Int(0) != right && let Primitive::Int(divisor) = left && let Primitive::Int(dividend) = right {
                            return Ok(Primitive::Int(divisor / dividend));
                        } else if Primitive::Float(0.) != left && let Primitive::Float(divisor) = left && let Primitive::Float(dividend) = right {
                            return Ok(Primitive::Float(divisor / dividend));
                        } else if Primitive::Int(0) == left || Primitive::Float(0.0) == left {
                            return Err(InterpException::DivideByZero(bin));
                        }
                        else {
                            return Err(InterpException::InvalidBinary(bin))
                        }
                    },
                    TokenType::STAR => {
                        if let Primitive::Int(num1) = left {
                            match right {
                                Primitive::Float(num2) => return Ok(Primitive::Float(num1 as f32 * num2)),
                                Primitive::Int(num2) => return Ok(Primitive::Int(num1 * num2)),
                                Primitive::String(_strng) => return Err(InterpException::InvalidBinary(bin)),
                                Primitive::Bool(boolean) => return Ok(Primitive::Int(num1 * Interpreter::convert_bool(boolean))),
                                _ => return Err(InterpException::InvalidBinary(bin)),
                            }
                        } else if let Primitive::Float(num1) = left && let Primitive::Float(num2) = right {
                            match right {
                                Primitive::Float(num2) => return Ok(Primitive::Float(num1 * num2)),
                                Primitive::Int(num2) => return Ok(Primitive::Float(num1 * num2 as f32)),
                                Primitive::String(_strng) => return Err(InterpException::InvalidBinary(bin)),
                                Primitive::Bool(boolean) => return Ok(Primitive::Float(num1 * Interpreter::convert_bool(boolean) as f32)),
                                _ => return Err(InterpException::InvalidBinary(bin)),
                            }
                        } else {
                            return Err(InterpException::InvalidBinary(bin))
                        }
                    },
                    TokenType::MINUS => {
                        if let Primitive::Int(num1) = left && let Primitive::Int(num2) = right {
                            return Ok(Primitive::Int(num1 - num2));
                        } else if let Primitive::Float(num1) = left && let Primitive::Float(num2) = right {
                            return Ok(Primitive::Float(num1 - num2));
                        } else {
                            return Err(InterpException::InvalidBinary(bin))
                        }
                    },
                    TokenType::PLUS => {
                        if let Primitive::Int(num1) = left {
                            match right {
                                Primitive::Float(num2) => return Ok(Primitive::Float(num1 as f32 + num2)),
                                Primitive::Int(num2) => return Ok(Primitive::Int(num1 + num2)),
                                Primitive::String(strng) => return Ok(Primitive::String(num1.to_string() + &strng)),
                                Primitive::Bool(boolean) => return Ok(Primitive::Int(num1 + Interpreter::convert_bool(boolean))),
                                _ => return Err(InterpException::InvalidBinary(bin)),
                            }
                        } else if let Primitive::Float(num1) = left {
                            match right {
                                Primitive::Float(num2) => return Ok(Primitive::Float(num1 + num2)),
                                Primitive::Int(num2) => return Ok(Primitive::Float(num1 + num2 as f32)),
                                Primitive::String(strng) => return Ok(Primitive::String(num1.to_string() + &strng)),
                                Primitive::Bool(boolean) => return Ok(Primitive::Float(num1 + Interpreter::convert_bool(boolean) as f32)),
                                _ => return Err(InterpException::InvalidBinary(bin)),
                            }
                        } else if let Primitive::String(str1) = left {
                            match right {
                                Primitive::Float(flt) => return Ok(Primitive::String(str1 + &flt.to_string())),
                                Primitive::Int(int) => return Ok(Primitive::String(str1 + &int.to_string())),
                                Primitive::String(str2) => return Ok(Primitive::String(str1 + str2.as_str())),
                                Primitive::Bool(boolean) => return Ok(Primitive::String(str1 + &boolean.to_string())),
                                _ => return Err(InterpException::InvalidBinary(bin)),
                            }
                        } else {
                            return Err(InterpException::InvalidBinary(bin))
                        }
                    },
                    TokenType::EQUAL_EQUAL => {
                        return Ok(Primitive::Bool(right == left));
                    },
                    TokenType::BANG_EQUAL => {
                        return Ok(Primitive::Bool(right != left))
                    },
                    TokenType::GREATER => {
                        return Ok(Primitive::Bool(left > right))
                    },
                    TokenType::GREATER_EQUAL => {
                        return Ok(Primitive::Bool(left >= right))
                    },
                    TokenType::LESS_EQUAL => {
                        return Ok(Primitive::Bool(left <= right))
                    },
                    TokenType::LESS => {
                        return Ok(Primitive::Bool(left < right))
                    }

                    _ => return Err(InterpException::InvalidBinary(bin))
                }
            },
            crate::ast::expr_types::ExprPossibilities::Grouping(group) => {
                return self.evaluate(&group.expr[0]);
            },
            crate::ast::expr_types::ExprPossibilities::Literal(lit) => {
                return Ok(lit.literal);
            },
            crate::ast::expr_types::ExprPossibilities::Ternary(ternary) => {
                let operator_res = self.evaluate(&ternary.condition)?;
                let bool_val = match operator_res {
                    Primitive::Float(flt) => flt != 0.0,
                    Primitive::Int(num) => num != 0,
                    Primitive::String(string) => string.len() != 0,
                    Primitive::Bool(bool_val) => bool_val,
                    Primitive::None => false,
                    _ => return Err(InterpException::PlaceHolder)
                };
                
                if bool_val && let Some(expr) = ternary.true_cond {
                    return self.evaluate(&expr);
                } else if !bool_val && let Some(expr) = ternary.false_cond {
                    return self.evaluate(&expr);
                } else {
                    return Ok(Primitive::None);
                }
            },
            ExprPossibilities::Stmt(stmt) => {
                match stmt.stmt {
                    TokenType::PRINT => {
                        unsafe {
                            let expr = self.evaluate(&stmt.inner.unwrap_unchecked())?;
                            match expr {
                                Primitive::Float(flt) => print!("{}", flt),
                                Primitive::Int(int) => print!("{}", int),
                                Primitive::String(strng) => print!("{}", strng),
                                Primitive::Bool(boolean) => print!("{}", boolean),
                                Primitive::Env(env) => print!("{:?}", env),
                                Primitive::None => print!("null"),
                                Primitive::Func(_) => return Err(InterpException::PlaceHolder)
                            }

                            return Ok(Primitive::None);
                        }
                    },
                    TokenType::PRINTLN => {
                        unsafe {
                                let expr = self.evaluate(&stmt.inner.unwrap_unchecked())?;
                                match expr {
                                    Primitive::Float(flt) => println!("{}", flt),
                                    Primitive::Int(int) => println!("{}", int),
                                    Primitive::String(strng) => println!("{}", strng),
                                    Primitive::Bool(boolean) => println!("{}", boolean),
                                    Primitive::Env(env) => println!("{:?}", env),
                                    Primitive::None => println!("null"),
                                    Primitive::Func(_) => return Err(InterpException::PlaceHolder),
                                }
                        }

                        return Ok(Primitive::None);
                    },
                    TokenType::LET => {
                        unsafe {
                            let expr = self.evaluate(&stmt.inner.unwrap_unchecked())?;
                            self.globals.define(&stmt.ident.unwrap_unchecked().lexeme, expr);
                            return Ok(Primitive::None)
                        }
                    },
                    TokenType::IDENTIFIER => {
                        unsafe {
                            return self.globals.retrieve(&stmt.ident.unwrap_unchecked().lexeme);
                        }
                    },
                    TokenType::FUNC => {
                        unsafe {
                            let ident = stmt.ident.unwrap_unchecked();
                            let func_data = self.globals.retrieve(&ident.lexeme)?;

                            if let Primitive::Func(func) = func_data {
                                let mut func_scope = self.enclose();
                                let inputted_params = stmt.params.unwrap_unchecked();
                                match func.func_map.get(&inputted_params.clone().len()) {
                                    Some((params, code))=> {
                                        for (idx, param_name) in params.iter().enumerate() {
                                            let prim = self.evaluate(&inputted_params[idx])?;
                                            if let Primitive::Env(env) = prim {
                                                func_scope.define_env(&param_name.lexeme, env.vars);
                                            } else {
                                                func_scope.define(&param_name.lexeme, prim);
                                            }
                                        }
                                        self.globals = func_scope.clone();
                                        for line in code.inner.iter() {
                                            self.evaluate(line)?;
                                        }

                                        self.globals = *func_scope.enclosing.unwrap_unchecked();
                                        todo!("Handle return here");
                                    },
                                    None => {
                                        self.globals = *func_scope.enclosing.unwrap_unchecked();
                                        return Err(InterpException::PlaceHolder)
                                    }
                                }
                            } else {
                                return Err(InterpException::PlaceHolder);
                            }
                        }
                    }
                    _ => return Err(InterpException::PlaceHolder)
                }
            }
            crate::ast::expr_types::ExprPossibilities::Unary(unary) => {
                let right = self.evaluate(unary.right.as_ref())?;

                match unary.operator.tok {
                    TokenType::MINUS | TokenType::BANG => {
                        match right {
                            Primitive::Float(float) => return Ok(Primitive::Float(-float)),
                            Primitive::Int(int) => return Ok(Primitive::Int(-int)),
                            Primitive::String(_string) => return Err(InterpException::InvalidUnary(unary)),
                            Primitive::Bool(boolean) => return Ok(Primitive::Bool(!boolean)),
                            Primitive::None => return Ok(Primitive::Bool(true)),
                            _ => return Err(InterpException::InvalidUnary(unary))
                        }
                    }
                    _  => {
                        return Err(InterpException::PlaceHolder);
                    }
                }
            },

            crate::ast::expr_types::ExprPossibilities::Scope(scope) => {
                match scope.stmt {
                    TokenType::FUNC => {
                        unsafe {
                            let close_ident = scope.ident.clone().unwrap_unchecked().clone();
                            if let Ok(Primitive::Func(func)) = self.globals.retrieve(&close_ident.lexeme) {
                                let mut func = func.clone();
                                let args = scope.params.clone().unwrap_unchecked();
                                let arg_len = args.len();
                                if let None = func.func_map.get(&arg_len) {
                                    func.func_map.insert(arg_len.clone(), (args, Box::new(scope)));
                                } else {
                                    return Err(InterpException::PlaceHolder);
                                }
                            } else {
                                let mut func_map = HashMap::new();
                                let params = scope.params.clone().unwrap_unchecked();
                                func_map.insert(params.len(), (params, Box::new(scope)));
                                self.globals.define(&close_ident.lexeme, Primitive::Func(Func { func_map }));
                            }
                            return Ok(Primitive::None);
                        }
                    },
                    TokenType::CLOS => {
                        unsafe {
                            let clos_ident = scope.ident.unwrap_unchecked().lexeme;
                            let mut clos_data: HashMap<String, Primitive> = HashMap::new();
                            for var in scope.inner {
                                if let ExprPossibilities::Stmt(var) = var {
                                    let var_ident = var.ident.unwrap_unchecked().lexeme;
                                    let val = self.evaluate(&var.inner.unwrap_unchecked())?;
                                    clos_data.insert(var_ident, val);
                                }
                            }
                            self.globals.define_env(&clos_ident, clos_data);
        
                            return Ok(Primitive::None);
                        }
                    },
                    TokenType::CLOSCALL => {
                        unsafe {
                            // println!("{:?}", scope);
                            let clos_ident = scope.ident.unwrap_unchecked().lexeme;
                            let data = self.globals.retrieve(&clos_ident)?;
                            if let Primitive::Env(env) = data {
                                let mut env = env;
                                    env.enclosing = Some(Box::new(self.globals.clone()));
                                    self.globals = env.clone();
                                    for line in scope.inner.iter() {
                                        self.evaluate(line)?;
                                    }
                                    self.globals = *env.enclosing.unwrap_unchecked();
                                    return Ok(Primitive::None);
                            } else {
                                return Err(InterpException::PlaceHolder);
                            }
                        }
                    },
                    TokenType::IF => {
                        let env = self.enclose();
                        for line in scope.inner.iter() {
                            self.evaluate(&line)?;
                        }

                        unsafe {
                            self.globals = *env.enclosing.unwrap_unchecked();
                            return Ok(Primitive::None);
                        }
                    }

                    TokenType::WHILE => {
                        let env = self.enclose();
                        unsafe {
                            while let Primitive::Bool(true) = self.evaluate(&scope.clone().condition.unwrap_unchecked())? {
                                for line in scope.inner.iter() {
                                    self.evaluate(line)?;
                                }
                            }
                            self.globals = *env.enclosing.unwrap_unchecked();
                        }
                        return Ok(Primitive::None);
                    },
                    TokenType::FOR => {
                        let env = self.enclose();
                        unsafe {
                            let cond = *scope.condition.unwrap_unchecked().clone();
                            if let ExprPossibilities::Grouping(group) = cond {
                                self.evaluate(&group.expr[0])?;
                                while let Primitive::Bool(true) = self.evaluate(&group.expr[1])? {
                                    for line in scope.inner.iter() {
                                        self.evaluate(line)?;
                                    }

                                    self.evaluate(&group.expr[2])?;
                                }
                            }

                            self.globals = *env.enclosing.unwrap_unchecked();
                        }

                        return Ok(Primitive::None);
                    }
                    _ => return Err(InterpException::PlaceHolder)
                }
                
            }
        }
    }
}

impl Interpreter {
    fn enclose(&mut self) -> Environment {
        let mut env: Environment = Environment::new();
        env.enclosing = Some(Box::new(self.globals.clone()));
        self.globals = env.clone();
        return env
    }
}