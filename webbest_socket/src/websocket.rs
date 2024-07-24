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
    pub fn connect(&mut self) -> Result<(), WebsocketError> {
        let mut buffer = [0_u8; 1024];

        let byte_length = match self.stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(e) => return Err(WebsocketError::new(e)),
        };

        let request = str::from_utf8(&buffer[..byte_length]).map_err(WebsocketError::new)?;

        if !request.starts_with("GET") {}
        Ok(())
    }
}

#[derive(Debug)]
pub struct WebsocketError {
    message: String,
}

impl WebsocketError {
    pub fn new(e: impl Error) -> WebsocketError {
        WebsocketError {
            message: e.to_string(),
        }
    }
}
impl Display for WebsocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "websocket error: {}", self.message)
    }
}

impl Error for WebsocketError {}
