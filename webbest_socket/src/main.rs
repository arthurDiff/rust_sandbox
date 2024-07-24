use std::{alloc::GlobalAlloc, net::TcpListener};

//https://www.thespatula.io/rust/rust_websocket/
fn main() {
    let addr = "127.0.0.1:8800";
    let listener = TcpListener::bind(addr).expect("Failed to bind to port 8800");
    println!("Websocket is listening on ws://{}", addr);

    for stream in listener.incoming() {}
}
