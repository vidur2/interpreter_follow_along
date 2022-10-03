use std::collections::HashMap;

use crate::{
    error_reporting::{interp_err::InterpException, parsing_err::ParsingException},
    scanner::token::Primitive,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub vars: HashMap<String, Primitive>,
    pub enclosing: Option<Box<Environment>>,
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
            return Ok(val.clone());
        } else if let Some(higher) = &self.enclosing {
            return higher.as_ref().retrieve(name);
        } else {
            return Err(InterpException::IdentifierNoExist(name.to_string()));
        }
    }
}
