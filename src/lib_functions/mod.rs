use self::list_ops::append;

pub mod list_ops;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LibFunctions{
    Append,
    Set,
    Len
}