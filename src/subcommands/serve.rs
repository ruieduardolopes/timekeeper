use std::io::Error;
use std::net::TcpListener;
use crate::server::handle_client;

pub fn init(port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }

    Ok(())
}