use indexmap::IndexMap;

pub mod decoder;
pub mod encoder;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Number(i128),
    // utf-8 str or others
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}
