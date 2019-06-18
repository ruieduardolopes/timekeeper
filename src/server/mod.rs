use chrono::offset::TimeZone;
use chrono::Utc;
use std::io::{Error, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let time = Utc::now().timestamp();

    let time_slice = &[
        ((time >> 24) & 0xFF) as u8,
        ((time >> 16) & 0xFF) as u8,
        ((time >> 8) & 0xFF) as u8,
        ((time >> 0) & 0xFF) as u8,
    ];

    stream.write(time_slice)?;

    Ok(())
}
