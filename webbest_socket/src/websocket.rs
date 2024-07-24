use core::str;
use std::{
    error::{self, Error},
    fmt::Display,
    io::{self, ErrorKind, Read},
    net::TcpStream,
};

pub struct Websocket {
    stream: TcpStream,
}

impl Websocket {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub fn connect(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mut buffer = [0_u8; 1024];

        let byte_length = match self.stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(e) => return Err(Box::new(WebsocketError::IoError(e))),
        };

        let request = str::from_utf8(&buffer[..byte_length])?;

        if !request.starts_with("GET") {}
        Ok(())
    }
}

#[derive(Debug)]
pub struct WebsocketError;

impl WebsocketError {
    pub fn IoError(e: io::Error) -> WebsocketError {
        WebsocketError
    }
}
impl Display for WebsocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed got error");
        Ok(())
    }
}

impl error::Error for WebsocketError {}
