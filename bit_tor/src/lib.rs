mod encoding;
mod torrent;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use encoding::{decoder::*, encoder::*};
pub use torrent::*;
