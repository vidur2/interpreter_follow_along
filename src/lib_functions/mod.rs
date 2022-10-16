use crate::interpreter::environment::Environment;

use self::math::Math;

pub mod list_ops;
pub mod math;
pub mod cast_ops;

pub const BUILTINS: [&str; 1] = ["math"];

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LibFunctions{
    Append,
    Set,
    Len,
    Slice,
    Math(MathLibFunctions),
    Int,
    Float,
    String
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