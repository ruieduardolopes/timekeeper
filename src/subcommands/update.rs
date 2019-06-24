use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use crate::utils::{from_slice, to_slice};
use crate::slave;

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    slave::init(address, port);

    Ok(())
}
