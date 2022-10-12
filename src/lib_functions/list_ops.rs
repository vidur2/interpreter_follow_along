use crate::{scanner::token::Primitive, error_reporting::interp_err::InterpException};

pub fn append(list: &mut Vec<Primitive>, new: Primitive) {
    list.push(new);
}

pub fn set(list: &mut Vec<Primitive>, idx: Primitive, primitive: Primitive) {
    if let Primitive::Int(idx_uw) = idx {
        list[idx_uw as usize] = primitive; 
    }
}

pub fn len(list: &Vec<Primitive>) -> Primitive {
    return Primitive::Int(list.len() as isize);
}