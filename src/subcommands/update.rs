use crate::slave;
use slog::Logger;
use std::io::Error;
use std::net::Ipv4Addr;

pub fn init(address: Ipv4Addr, port: u16, log: Logger) -> Result<(), Error> {
    slave::init(address, port, log.clone());

    Ok(())
}
