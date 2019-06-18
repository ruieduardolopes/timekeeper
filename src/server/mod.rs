use std::io::{Error, Write};
use std::net::TcpStream;
use chrono::{Utc, DateTime};
use chrono::offset::TimeZone;

pub fn handle_client(stream: &mut TcpStream) -> Result<(), Error> {
    let time = DateTime::timestamp();
    stream.write(time)?;

    Ok(())
}