use crate::master::handle_client;
use slog::{error, info, Logger};
use std::io::{Error, ErrorKind};
use std::net::TcpListener;
use std::thread;

pub fn init(port: u16, log: Logger) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("10.1.1.3:{}", port));
    let listener = match listener {
        Ok(listener) => {listener},
        Err(error) => {match error.kind() {
            ErrorKind::PermissionDenied => {error!(log, "[timekeeper] Permission denied, while opening port {}", port)},
            ErrorKind::ConnectionRefused => {error!(log, "[timekeeper] Connection refused on port {}", port)},
            ErrorKind::ConnectionReset => {error!(log, "[timekeeper] Connection reset on port {}", port)},
            ErrorKind::ConnectionAborted => {error!(log, "[timekeeper] Connection aborted on port {}", port)},
            ErrorKind::AlreadyExists => {error!(log, "[timekeeper] Already exists something running on port {}", port)},
            _ => {},
        };
        panic!("{}", error)},
    };
    info!(
        log,
        "[timekeeper] Initializing timekeeper as master, listening in port {}", port
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!(
                    log,
                    "[timekeeper] A new slave has communicated. Starting protocol..."
                );
                thread::spawn({
                    let log = log.clone();
                    move || handle_client(stream, log)
                });
            }
            Err(_) => {
                panic!("Connection's failed");
            }
        }
    }

    Ok(())
}
