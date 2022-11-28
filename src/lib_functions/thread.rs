use std::{thread, sync::{Arc, Mutex}};

use crate::{interpreter::{environment::Environment, interpreter::Interpreter}, error_reporting::interp_err::InterpException, scanner::token::{Primitive, Token}};

const FUT_KEY: &str = "returned";

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Thread {
    Spawn
}

pub fn new() -> Environment {
    let mut ret_env = Environment::new();
    ret_env.define("spawn", Primitive::NativeFunc(super::NativeFunc::Thread(Thread::Spawn)));
    return ret_env;
}
pub fn spawn(target_env: Arc<Mutex<Environment>>, func_ptr: &str, args_len: usize, callback_ptr: String) -> Result<(), InterpException> {
    let mut env = target_env.lock().unwrap();
    let func = env.retrieve(func_ptr)?;
    env.define(FUT_KEY, Primitive::None);
    drop(env);
    if let Primitive::Func(func) = func {
        let mut interpreter = Interpreter::new();
        interpreter.globals = Arc::new(Mutex::new(Environment::new()));
        interpreter.globals.lock().unwrap().enclosing = Some(Box::new(target_env));
        thread::spawn(move || {
            let mut result: Primitive = Primitive::None;
            for expr in func.func_map.get(&args_len).unwrap().1.as_ref().inner.iter() {
                result = interpreter.evaluate(expr).unwrap();
            }

            let mut multi = interpreter.globals.lock().unwrap();
            multi.redefine(FUT_KEY, result.clone()).unwrap();
            let callback = multi.retrieve(&callback_ptr).unwrap();
            drop(multi);
            if let Primitive::Func(callback) = callback {
                let code = &callback.func_map.get(&1).unwrap().1.as_ref().inner;
                let param_name = &callback.func_map.get(&1).unwrap().0[0].lexeme;
                let mut env = Environment::new();
                env.define(param_name, result);
                interpreter.globals = Arc::new(Mutex::new(env));
                for expr in code.iter() {
                    interpreter.evaluate(expr).unwrap();
                }
            }
        });
    }

    return Ok(());
}
