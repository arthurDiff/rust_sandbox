use std::{fs::File, path::PathBuf};

//https://ngoldbaum.github.io/posts/loading-mnist-data-in-rust/
#[derive(Debug)]
pub struct Mnist {}

impl Mnist {
    pub fn new(file: File) -> Self {
        let mut gz = flate2::read::GzDecoder::new(file);
        todo!()
    }
}
