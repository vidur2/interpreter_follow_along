use std::collections::HashMap;

use crate::{scanner::token::Primitive, error_reporting::{parsing_err::ParsingException, interp_err::InterpException}};

#[derive(Clone, Debug)]
pub enum EnvironmentOption{
    Primitive(Primitive),
    Environment(Environment)
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub vars: HashMap<String, EnvironmentOption>,
    pub enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            vars: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: &str, value: Primitive) {
        self.vars.insert(name.to_string(), EnvironmentOption::Primitive(value));
    }

    pub fn define_env(&mut self, name: &str, vars: HashMap<String, EnvironmentOption>) {
        let env: Environment = Environment { vars, enclosing: None };
        self.vars.insert(name.to_string(), EnvironmentOption::Environment(env));
    }


    pub fn retrieve(&self, name: &str) -> Result<EnvironmentOption, InterpException> {
        if let Some(val) = self.vars.get(name) {
            return Ok(val.clone());
        } else if let Some(higher) = &self.enclosing {
            return higher.as_ref().retrieve(name);
        }else {
            return Err(InterpException::IdentifierNoExist(name.to_string()));
        }
    }
}