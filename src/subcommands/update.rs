use std::io::Error;
use std::net::{Ipv4Addr, TcpStream};

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    stream

    Ok(())
}