use crate::scanner::token::Primitive;

pub fn int(other: Primitive) -> Primitive {
    match other {
        Primitive::Float(flt) => return Primitive::Int(flt as isize),
        Primitive::Int(_) => return other,
        Primitive::String(string) => {
            let parsed_res: Result<isize, _> = string.trim().parse();
            match parsed_res {
                Ok(val) => return Primitive::Int(val),
                Err(_) => return Primitive::None,
            }
        },
        Primitive::Bool(boolean) => return Primitive::Int(boolean as isize),
        Primitive::Env(_) => return Primitive::None,
        Primitive::Func(_) => return Primitive::None,
        Primitive::NativeFunc(_) => return Primitive::None,
        Primitive::List(_) => return Primitive::None,
        Primitive::None => return Primitive::None,
    }
}

pub fn string(other: Primitive) -> Primitive {
    match other {
        Primitive::Float(flt) => return Primitive::String(flt.to_string()),
        Primitive::Int(int) => return Primitive::String(int.to_string()),
        Primitive::String(_) => return other,
        Primitive::Bool(boolean) => return Primitive::String(boolean.to_string()),
        Primitive::Env(env) => return Primitive::String(format!("{:?}", env)),
        Primitive::Func(_) => return Primitive::None,
        Primitive::NativeFunc(_) => return Primitive::None,
        Primitive::List(list) => return Primitive::String(format!("{:?}", list)),
        Primitive::None => return Primitive::String("null".to_string()),
    }
}