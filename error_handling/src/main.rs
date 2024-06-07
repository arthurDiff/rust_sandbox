use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn readline_from_file(name: String) -> Result<String, io::Error> {
    let mut string_to_pass = String::new();
    // let mut file = File::open(name)?;
    // file.read_to_string(&mut string_to_pass)?;
    File::open(name)?.read_to_string(&mut string_to_pass)?;
    //fs::read_to_string(name)
    Ok(string_to_pass)
}
