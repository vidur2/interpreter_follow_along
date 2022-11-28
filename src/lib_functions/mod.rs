use crate::interpreter::environment::Environment;

use self::{math::Math, thread::{Thread, new}};

pub mod cast_ops;
pub mod list_ops;
pub mod math;
pub mod thread;

pub const BUILTINS: [&str; 2] = ["math", "thread"];

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NativeFunc {
    Append,
    Set,
    Len,
    Slice,
    Math(MathLibFunctions),
    Thread(Thread),
    Int,
    Float,
    String
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MathLibFunctions {
    Sin,
    Cos,
    Tan,
}

pub fn import_lib(import_name: &str) -> Environment {
    match import_name {
        "math" => {
            return Math::new();
        },
        "thread" => {
            return new();
        }
        _ => todo!(),
    }
}
