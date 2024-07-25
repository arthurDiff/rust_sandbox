use std::{error::Error, fmt::Display};

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
    pub fn from_string(message: String) -> WebsocketError {
        WebsocketError { message }
    }
}
impl Display for WebsocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "websocket error: {}", self.message)
    }
}

impl Error for WebsocketError {}
