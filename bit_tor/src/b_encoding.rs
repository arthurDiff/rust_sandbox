use indexmap::IndexMap;

pub mod decoder;
pub mod encoder;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(i128),
    String(String),
    List(Vec<Value>),
    Object(IndexMap<String, Value>),
}
