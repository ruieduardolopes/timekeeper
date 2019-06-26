use crate::master::handle_client;
use std::io::Error;
use std::net::TcpListener;
use std::thread;
use slog::Logger;

pub fn init(port: u16, log: Logger) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    info!(log, "[timekeeper] Initializing timekeeper as master, listening in port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!(log, "[timekeeper] A new slave has communicated. Starting protocol...");
                thread::spawn(move || handle_client(stream));
            }
            Err(_) => {
                panic!("Connection's failed");
            }
        }
    }

    Ok(())
}
