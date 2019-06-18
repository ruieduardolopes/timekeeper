use std::io::Error;
use std::net::TcpListener;
use crate::server::handle_client;
use std::thread;

pub fn init(port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO log a new connection
                thread::spawn(move ||
                    handle_client(stream)
                );
            },
            Err(_) => {},
        }
    }

    Ok(())
}