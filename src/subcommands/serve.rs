use crate::master::handle_client;
use std::io::Error;
use std::net::TcpListener;
use std::thread;

pub fn init(port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!("listener is {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO log a new connection
                thread::spawn(move || handle_client(stream));
            }
            Err(_) => {
                panic!("Connection's failed");
            }
        }
    }

    Ok(())
}
