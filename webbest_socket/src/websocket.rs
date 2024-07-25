use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use base64::Engine as _;
use websocket_error::WebsocketError;

mod websocket_error;
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

        if !request.starts_with("GET") {
            return Err(WebsocketError::from_string(
                "Require it to be GET request".into(),
            ));
        }

        let response = self.handle_handshake(request)?;
        self.stream
            .write_all(response.as_bytes())
            .map_err(WebsocketError::new)?;

        self.stream.flush().map_err(WebsocketError::new)?;
        Ok(())
    }

    pub fn handle_handshake(&self, req: &str) -> Result<String, WebsocketError> {
        let key_header = "Sec-Websocket-Key";

        let key = req
            .lines()
            .find(|l| l.starts_with(key_header))
            .map(|l| l[key_header.len()..].trim())
            .ok_or_else(|| {
                WebsocketError::from_string(
                    "Couldn't find Sec-Websocket-key header in http reqeust".into(),
                )
            })?;
        let response_key = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key);

        let hash = sha256::digest(response_key);
        let header_key = base64::engine::general_purpose::STANDARD.encode(hash);
        Ok(format!(
            "HTTP/1.1 101 Switching Protocols\r\n
            Upgrade: websocket\r\n
            Connection: Upgrade\r\n
            Sec-Websocket-Accept: {}\r\n\r\n
            ",
            header_key
        ))
    }

    pub fn handle_connection(&mut self) -> Result<(), WebsocketError> {
        let mut buffer = [0; 2048];

        self.send_ping()?;
        let mut last_ping = std::time::Instant::now();
        let mut pong_received = false;

        loop {
            if last_ping.elapsed() > Duration::from_secs(10) {
                if !pong_received {
                    println!("Pong not received: disconnecting client.");
                    break;
                }

                if self.send_ping().is_err() {
                    println!("Ping failed: disconnecting client.");
                    break;
                }

                pong_received = false;
                last_ping = std::time::Instant::now();
            }

            match self.stream.read(&mut buffer) {
                Ok(n) if n > 0 => match self.parse_frame(&buffer[..n]) {
                    Ok(f) => match f {
                        Frame::Text(d) => match String::from_utf8(d) {
                            Ok(text) => {
                                println!("Received data: {}", text);
                                if self.send_text(&text).is_err() {
                                    println!("Failed to send msg");
                                    break;
                                }
                            }
                            Err(e) => return Err(WebsocketError::new(e)),
                        },
                        Frame::Binary(d) => println!("{:?}", d),
                        Frame::Close => println!("Close request"),
                        Frame::Ping => println!("ping"),
                        Frame::Pong => println!("pong"),
                    },
                    Err(e) => return Err(WebsocketError::new(e)),
                },
                Ok(_) => {
                    return Err(WebsocketError::from_string("Frame too short".to_string()));
                }
                Err(e) => return Err(WebsocketError::new(e)),
            };
        }

        Ok(())
    }
    pub fn send_ping(&self) -> Result<(), WebsocketError> {
        Ok(())
    }
    pub fn send_text(&self, msg: &str) -> Result<(), WebsocketError> {
        todo!();
    }
    pub fn parse_frame(&self, buffer: &[u8]) -> Result<Frame, WebsocketError> {
        if buffer.len() < 2 {
            return Err(WebsocketError::from_string("Frame too short".to_string()));
        }
        let (first_byte, second_byte) = (buffer[0], buffer[1]);
        let opcode = first_byte & 0x0f;
        let masked = (second_byte & 0x80) != 0;
        let mut payload_len = (second_byte & 0x7F) as usize;

        if !masked {
            return Err(WebsocketError::from_string(
                "Frames from client must be masked".to_string(),
            ));
        }

        let mut offset = 2;

        if payload_len == 126 {
            if buffer.len() < 4 {
                return Err(WebsocketError::from_string(
                    "Froame too short for extended payload length".to_string(),
                ));
            }

            payload_len = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]) as usize;
            offset += 2;
        } else if payload_len == 127 {
            return Err(WebsocketError::from_string(
                "Extended payload length too large".to_string(),
            ));
        }

        if buffer.len() < offset + 4 + payload_len {
            return Err(WebsocketError::from_string(
                "Frame too short for mask and data".to_string(),
            ));
        }

        let mask = &buffer[offset..offset + 4];

        offset += 4;

        let mut data = Vec::with_capacity(payload_len);
        for i in 0..payload_len {
            data.push(buffer[offset + i] ^ mask[i % 4]);
        }

        Ok(match opcode {
            0x01 => Frame::Text(data),
            0x02 => Frame::Binary(data),
            0x08 => Frame::Close,
            0x09 => Frame::Ping,
            0x0A => Frame::Pong,
            _ => return Err(WebsocketError::from_string("Unknown opcode".to_string())),
        })
    }
}

pub enum Frame {
    Text(Vec<u8>),
    Binary(Vec<u8>),
    Close,
    Ping,
    Pong,
}
