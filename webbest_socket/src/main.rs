use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use websocket::Websocket;
mod websocket;
//https://www.thespatula.io/rust/rust_websocket/
fn main() {
    let addr = "127.0.0.1:8800";
    let listener = TcpListener::bind(addr).expect("Failed to bind to port 8800");
    println!("Websocket is listening on ws://{}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_client(s);
                });
            }
            Err(e) => {
                println!("Failed to accept stream: {:?}", e);
            }
        }
    }
}

fn handle_client(s: TcpStream) {
    let mut ws = Websocket::new(s);
    ws.connect();
}
