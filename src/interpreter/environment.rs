use std::{collections::HashMap, sync::{Mutex, Arc}};

use crate::{
    error_reporting::{interp_err::InterpException, parsing_err::ParsingException},
    scanner::token::Primitive,
};

#[derive(Clone, Debug)]
pub struct Environment {
    pub vars: HashMap<String, Primitive>,
    pub enclosing: Option<Box<Arc<Mutex<Environment>>>>,
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.vars == other.vars
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_len = self.vars.len();
        let other_len = other.vars.len();
        if my_len > other_len {
            return Some(std::cmp::Ordering::Greater);
        } else if my_len < other_len {
            return Some(std::cmp::Ordering::Less);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            vars: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: &str, value: Primitive) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn define_env(&mut self, name: &str, vars: HashMap<String, Primitive>) {
        let env: Environment = Environment {
            vars,
            enclosing: None,
        };
        self.vars.insert(name.to_string(), Primitive::Env(env));
    }

    pub fn retrieve(&self, name: &str) -> Result<Primitive, InterpException> {
        if let Some(val) = self.vars.get(name) {
            // println!("1: {}", name);
            return Ok(val.clone());
        } else if let Some(higher) = &self.enclosing {
            // println!("2: {}", name);
            return higher.lock().unwrap().retrieve(name);
        } else {
            // println!("3: {}", name);
            return Err(InterpException::IdentifierNoExist(name.to_string()));
        }
    }

    pub fn redefine(&mut self, name: &str, value: Primitive) -> Result<(), InterpException> {
        if let Some(_) = self.vars.get(name) {
            if let Primitive::Env(env) = value {
                self.define_env(name, env.vars.clone());
            } else {
                self.define(name, value);
            }
            return Ok(());
        } else if let Some(mut enc) = self.enclosing.clone() {
            enc.as_mut().lock().unwrap().redefine(name, value)?;
            self.enclosing = Some(enc);
            return Ok(());
        } else {
            return Err(InterpException::IdentifierNoExist(name.to_string()));
        }
    }
}
