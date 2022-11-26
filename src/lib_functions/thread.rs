use std::{thread, sync::{Arc, Mutex}};

use crate::{interpreter::{environment::Environment, interpreter::Interpreter}, error_reporting::interp_err::InterpException, scanner::token::{Primitive, Token}};

pub fn new() -> Environment {
    let mut ret_env = Environment::new();
    ret_env.define("futures", Primitive::List(Vec::new()));
    return ret_env;
}
pub fn spawn(thread_env: Arc<Mutex<Environment>>, target_env: Arc<Mutex<Environment>>, func_ptr: &str, args_len: usize) -> Result<(), InterpException> {
    let func = target_env.lock().unwrap().retrieve(func_ptr)?;
    if let Primitive::Func(func) = func {
        let mut interpreter = Interpreter::new();
        interpreter.globals = Arc::new(Mutex::new(Environment::new()));
        interpreter.globals.lock().unwrap().enclosing = Some(Box::new(target_env));
        thread::spawn(move || {
            let mut result: Result<Primitive, InterpException>;
            // if let Primitive::List(list) = thread_env.lock().unwrap().retrieve("futures").unwrap() {
            //     list.push()
            // }
            for expr in func.func_map.get(&args_len).unwrap().1.as_ref().inner.iter() {
                result = interpreter.evaluate(expr);
            }

            
        });
    }

    return Ok(());
}
