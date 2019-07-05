use crate::slave;
use slog::Logger;
use std::io::Error;
use std::net::Ipv4Addr;

pub fn init(address: Ipv4Addr, port: u16, dry_run: bool, log: Logger) -> Result<(), Error> {
    slave::init(address, port, dry_run, log.clone());

    Ok(())
}
