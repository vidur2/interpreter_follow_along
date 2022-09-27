use std::collections::HashMap;

use crate::{scanner::token::Primitive, error_reporting::{parsing_err::ParsingException, interp_err::InterpException}};

#[derive(Clone, Debug)]
pub enum EnvironmentOption{
    Primitive(Primitive),
    Environment(Environment)
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub vars: HashMap<String, EnvironmentOption>
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            vars: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: EnvironmentOption) {
        self.vars.insert(name.to_string(), value);
    }


    pub fn retrieve(&self, name: &str) -> Result<EnvironmentOption, InterpException> {
        if let Some(val) = self.vars.get(name) {
            return Ok(val.clone());
        } else {
            return Err(InterpException::IdentifierNoExist(name.to_string()));
        }
    }
}