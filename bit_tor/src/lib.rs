mod encoding;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use encoding::{decoder::*, encoder::*};
