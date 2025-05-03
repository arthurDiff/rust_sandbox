use indexmap::IndexMap;

pub mod decoder;
pub mod encoder;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Number(i128),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}
