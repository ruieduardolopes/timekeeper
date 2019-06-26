use crate::slave;
use crate::utils::{from_slice, to_slice};
use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use slog::Logger;

pub fn init(address: Ipv4Addr, port: u16, log: Logger) -> Result<(), Error> {
    slave::init(address, port, log.clone());

    Ok(())
}
