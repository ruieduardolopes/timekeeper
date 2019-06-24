use chrono::offset::TimeZone;
use chrono::Utc;
use std::io::{Error, Write, Read};
use std::net::TcpStream;
use crate::utils::{to_slice, from_slice};

pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // Grab the most updated timestamp value from the server and convert it to a slice.
    let time = Utc::now().timestamp_millis();
    let time_slice = &to_slice(time);

    // Send the current server time to the client.
    stream.write(time_slice)?;

    // Receive offset from client, to the sent timestamp.
    let mut offset_slice_from_client: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut offset_slice_from_client)?;
    let offset_from_client = from_slice(&offset_slice_from_client);

    // TODO estimate all offsets and retrieve a correction
    let
    let correction: i64 = 0;

    // Pass the offset correction to slice and send it to the client.
    let correction_slice = &to_slice(correction);
    stream.write(correction_slice);

    Ok(())
}
