use std::thread;

use crate::{interpreter::{environment::Environment, interpreter::Interpreter}, error_reporting::interp_err::InterpException, scanner::token::{Primitive, Token}};

pub fn new() -> Environment {
    let mut ret_env = Environment::new();
    ret_env.define("futures", Primitive::List(Vec::new()));
    return ret_env;
}
pub fn spawn(thread_env: &mut Environment, target_env: &mut Environment, func_ptr: &str, args_len: usize) -> Result<(), InterpException> {
    let func = target_env.retrieve(func_ptr)?;
    if let Primitive::Func(func) = func {
        let mut interpreter = Interpreter::new();
        interpreter.globals = Environment::new();
        interpreter.globals.enclosing = Some(Box::new(target_env.clone()));
        thread::spawn(move || {
            let mut result: Result<Primitive, InterpException>;
            for expr in func.func_map.get(&args_len).unwrap().1.as_ref().inner.iter() {
                result = interpreter.evaluate(expr);
            }

            target_env = &mut interpreter.globals.enclosing.unwrap();
        });
    }

    return Ok(());
}
