use chrono::offset::TimeZone;
use chrono::Utc;
use std::io::{Error, Write, Read};
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

    let mut offset_slice_from_client: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut offset_slice_from_client)?;

    // TODO estimate all offsets and retrieve a correction

    let correction: i64 = 0;
    let correction_slice = &[
        ((correction >> 24) & 0xFF) as u8,
        ((correction >> 16) & 0xFF) as u8,
        ((correction >> 8) & 0xFF) as u8,
        ((correction >> 0) & 0xFF) as u8,
    ];
    stream.write(correction_slice);

    Ok(())
}
