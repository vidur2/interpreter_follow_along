use crate::{interpreter::environment::Environment, scanner::token::Primitive, ast::expr_types::ExprPossibilities, error_reporting::interp_err::InterpException};

use super::{LibFunctions, MathLibFunctions};

pub struct Math  {
    env: Environment,
}

impl Math {
    pub fn new() -> Environment {
        let mut ret_env = Environment::new();

        ret_env.define("sin", Primitive::NativeFunc(LibFunctions::Math(MathLibFunctions::Sin)));
        ret_env.define("cos", Primitive::NativeFunc(LibFunctions::Math(MathLibFunctions::Cos)));
        ret_env.define("tan", Primitive::NativeFunc(LibFunctions::Math(MathLibFunctions::Tan)));
        return ret_env;
    }

    pub fn do_func(func_name: MathLibFunctions, param: Vec<Result<Primitive, InterpException>>) -> Result<Primitive, InterpException> {
        match func_name {
            MathLibFunctions::Sin => Math::trig_op(param, func_name),
            MathLibFunctions::Cos => Math::trig_op(param, func_name),
            MathLibFunctions::Tan => Math::trig_op(param, func_name),
        }
    }

    fn trig_op(params: Vec<Result<Primitive, InterpException>>, func_name: MathLibFunctions) -> Result<Primitive, InterpException> {
        if let Primitive::Int(angle) = params[0].clone()? {
            match func_name {
                MathLibFunctions::Sin => return Ok(Primitive::Float((angle as f32).sin())),
                MathLibFunctions::Cos => return Ok(Primitive::Float((angle as f32).cos())),
                MathLibFunctions::Tan => return Ok(Primitive::Float((angle as f32).tan())),
            }
        } else if let Primitive::Float(angle) = params[0].clone()? {
            match func_name {
                MathLibFunctions::Sin => return Ok(Primitive::Float(angle.sin())),
                MathLibFunctions::Cos => return Ok(Primitive::Float(angle.cos())),
                MathLibFunctions::Tan => return Ok(Primitive::Float(angle.tan())),
            }
        } 
        return Err(InterpException::PlaceHolder)
    }
}