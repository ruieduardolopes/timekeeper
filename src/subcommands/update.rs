use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    let mut time_slice_from_server: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut time_slice_from_server)?;

    let current_time = Utc::now().timestamp();

    let time_from_server = ((time_slice_from_server[0] as i64) << 24)
        + ((time_slice_from_server[1] as i64) << 16)
        + (((time_slice_from_server[2]) as i64) << 8)
        + ((time_slice_from_server[3]) as i64);

    let time_difference = current_time - time_from_server;

    let difference_slice = &[
        ((time_difference >> 24) & 0xFF) as u8,
        ((time_difference >> 16) & 0xFF) as u8,
        ((time_difference >> 8) & 0xFF) as u8,
        ((time_difference >> 0) & 0xFF) as u8,
    ];
    stream.write(difference_slice)?;

    let mut time_correction: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut time_correction)?;
    let correction = ((time_correction[0] as i64) << 24)
        + ((time_correction[1] as i64) << 16)
        + (((time_correction[2]) as i64) << 8)
        + ((time_correction[3]) as i64);

    // TODO adjust this clock by correction

    Ok(())
}
