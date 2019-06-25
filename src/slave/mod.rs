use crate::messages::PTPMessage;
use crate::time_adjuster::*;
use crate::utils::*;
use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use libc::*;
use time::Timespec;

pub fn init(address: Ipv4Addr, port: u16) -> Result<(), Error> {
    // Connect with timekeeper server.
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    // Wait and receive Sync message from master.
    let mut sync_message: [u8; 4] = [0; 4];
    stream.read(&mut sync_message)?;
    let time_on_sync_received = time::now().to_timespec();

    // Wait and receive Follow-Up message from master.
    let mut follow_up_message: [u8; 16] = [0; 16];
    stream.read(&mut follow_up_message)?;
    let time_on_sync_from_master = timespec_from_slice(array_ref!(follow_up_message, 4, 16));

    // Evaluate the offset from this first stage.
    let main_offset = time_on_sync_from_master - time_on_sync_received;

    // Adjust the internal clock with such offset.
    set_time_by_offset(main_offset);

    // Send Delay Request to Master.
    let delay_request_message = &to_slice(PTPMessage::DelayRequest as i64);
    stream.write(delay_request_message)?;
    let time_on_delay_request = time::now().to_timespec();

    // Wait and receive Delay Response from master.
    let mut delay_reponse_message: [u8; 16] = [0; 16];
    stream.read(&mut delay_reponse_message)?;
    let time_on_delay_request_from_master = timespec_from_slice(array_ref!(delay_reponse_message, 4, 16));

    // Evaluate the propagation delay.
    let propagation_delay = time_on_delay_request_from_master - time_on_delay_request;
    let propagation_delay = propagation_delay / 2;

    // Adjust the internal clock with such offset.
    set_time_by_offset(propagation_delay);

    Ok(())
}
