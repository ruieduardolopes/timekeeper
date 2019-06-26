use crate::adjuster::*;
use crate::messages::PTPMessage;
use crate::utils::*;
use slog::{info, Logger};
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub fn init(address: Ipv4Addr, port: u16, log: Logger) -> Result<(), Error> {
    // Connect with timekeeper server.
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;
    info!(
        log,
        "[timekeeper] Connection made with master at {} in port {}", address, port
    );

    // Wait and receive Sync message from master.
    let mut sync_message: [u8; 4] = [0; 4];
    info!(
        log,
        "[timekeeper] Waiting for a Sync message from master..."
    );
    stream.read(&mut sync_message)?;
    let time_on_sync_received = time::now().to_timespec();
    info!(
        log,
        "[timekeeper] Received Sync message from master at {}.{} seconds",
        time_on_sync_received.sec,
        time_on_sync_received.nsec
    );

    // Wait and receive Follow-Up message from master.
    let mut follow_up_message: [u8; 16] = [0; 16];
    info!(
        log,
        "[timekeeper] Waiting for a Follow-Up message from master..."
    );
    stream.read(&mut follow_up_message)?;
    let time_on_sync_from_master = timespec_from_slice(array_ref!(follow_up_message, 4, 16));
    info!(
        log,
        "[timekeeper] Received Follow-Up message from master with timestamp {}.{} seconds",
        time_on_sync_from_master.sec,
        time_on_sync_from_master.nsec
    );

    // Evaluate the offset from this first stage.
    let main_offset = time_on_sync_from_master - time_on_sync_received;
    info!(
        log,
        "[timekeeper] The main offset is of {} seconds",
        main_offset.num_seconds()
    );

    // Adjust the internal clock with such offset.
    set_time_by_offset(main_offset, log.clone());

    // Send Delay Request to Master.
    let delay_request_message = &to_slice(PTPMessage::DelayRequest as i64);
    info!(
        log,
        "[timekeeper] Sending Delay Request message to master..."
    );
    stream.write(delay_request_message)?;
    let time_on_delay_request = time::now().to_timespec();
    info!(
        log,
        "[timekeeper] Marked {}.{} as timestamp on sending Delay Request message",
        time_on_delay_request.sec,
        time_on_delay_request.nsec
    );

    // Wait and receive Delay Response from master.
    let mut delay_reponse_message: [u8; 16] = [0; 16];
    info!(
        log,
        "[timekeeper] Waiting for a Delay Response message from master..."
    );
    stream.read(&mut delay_reponse_message)?;
    let time_on_delay_request_from_master =
        timespec_from_slice(array_ref!(delay_reponse_message, 4, 16));
    info!(
        log,
        "[timekeeper] Received Delay Response message from master with timestamp {}.{} seconds",
        time_on_delay_request_from_master.sec,
        time_on_delay_request_from_master.nsec
    );

    // Evaluate the propagation delay.
    let propagation_delay = time_on_delay_request_from_master - time_on_delay_request;
    let propagation_delay = propagation_delay / 2;
    info!(
        log,
        "[timekeeper] Estimated {} seconds of propagation delay. Fixing...",
        propagation_delay.num_seconds()
    );

    // Adjust the internal clock with such offset.
    set_time_by_offset(propagation_delay, log.clone());

    Ok(())
}
