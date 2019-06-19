use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use crate::utils::{from_slice, to_slice};

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    // Connect to timekeeper server.
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    // Read time from server.
    let mut time_slice_from_server: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut time_slice_from_server)?;

    // Get current time on client.
    let current_time = Utc::now().timestamp_millis();

    // Pass server time to slice.
    let time_from_server = from_slice(&time_slice_from_server);

    // Estimate the delta between the client time and server's.
    let time_delta = current_time - time_from_server;
    let delta_slice = &to_slice(time_delta);

    // Send the delta to server.
    stream.write(delta_slice)?;

    // Retrieve time correction, from server.
    let mut time_correction: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut time_correction)?;
    let correction = from_slice(&time_correction);

    // TODO adjust this clock by correction

    Ok(())
}
