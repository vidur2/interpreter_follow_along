use crate::scanner::token::Primitive;

pub fn append(list: &mut Vec<Primitive>, new_func: Primitive) {
    list.push(new_func);
}