use chrono::Utc;
use std::io::{Error, Read};
use std::net::{Ipv4Addr, TcpStream};

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    let mut time_slice_from_server: [u8; 4] = [0, 0, 0, 0];
    stream.read(&mut time_slice_from_server)?;

    let time_from_server = ((time_slice_from_server[0] as i64) << 24)
        + ((time_slice_from_server[1] as i64) << 16)
        + (((time_slice_from_server[2]) as i64) << 8)
        + ((time_slice_from_server[3]) as i64);

    let current_time = Utc::now().timestamp();

    Ok(())
}
