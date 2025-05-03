mod b_encoding;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use b_encoding::{decoder::*, encoder::*};
