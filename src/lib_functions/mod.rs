use crate::interpreter::environment::Environment;

use self::{list_ops::append, math::Math};

pub mod list_ops;
pub mod math;

pub const BUILTINS: [&str; 1] = ["math"];

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LibFunctions{
    Append,
    Set,
    Len,
    Slice,
    Math(MathLibFunctions),
}


#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MathLibFunctions {
    Sin,
    Cos,
    Tan
}

pub fn import_lib(import_name: &str) -> Environment{
    match import_name {
        "math" => {
            return Math::new();
        }
        _ => todo!(),
    }
}