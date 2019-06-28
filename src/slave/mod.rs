use crate::adjuster::*;
use crate::messages::PTPMessage;
use crate::utils::*;
use slog::{error, info, Logger};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub fn init(address: Ipv4Addr, port: u16, log: Logger) -> Result<(), Error> {
    // Connect with timekeeper server.
    info!(log, "[timekeeper] Trying to connect {}:{}.", address, port);
    let mut stream = TcpStream::connect(&format!("{}:{}", address, port));
    let mut stream = match stream {
        Ok(stream) => stream,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    error!(log, "[timekeeper] Endpoint {}:{} not found", address, port);
                }
                ErrorKind::PermissionDenied => {
                    error!(
                        log,
                        "[timekeeper] Permission denied on connecting with {}:{}", address, port
                    );
                }
                ErrorKind::ConnectionRefused => {
                    error!(
                        log,
                        "[timekeeper] Connection refused on connecting with {}:{}", address, port
                    );
                }
                ErrorKind::ConnectionReset => {
                    error!(
                        log,
                        "[timekeeper] Connection reset with {}:{}", address, port
                    );
                }
                ErrorKind::ConnectionAborted => {
                    error!(
                        log,
                        "[timekeeper] Connection aborted with {}:{}", address, port
                    );
                }
                ErrorKind::NotConnected => {
                    error!(log, "[timekeeper] Not connected with {}:{}", address, port);
                }
                ErrorKind::AddrInUse => {
                    error!(log, "[timekeeper] Address already in use");
                }
                ErrorKind::AddrNotAvailable => {
                    error!(log, "[timekeeper] Address not available");
                }
                ErrorKind::BrokenPipe => {
                    error!(log, "[timekeeper] Broken pipe with {}:{}", address, port);
                }
                ErrorKind::AlreadyExists => {
                    error!(log, "[timekeeper] Already exists");
                }
                ErrorKind::WouldBlock => {
                    error!(log, "[timekeeper] Connection would block");
                }
                ErrorKind::TimedOut => {
                    error!(
                        log,
                        "[timekeeper] Connection timed out with {}:{}.", address, port
                    );
                }
                ErrorKind::Other => {}
                ErrorKind::UnexpectedEof => {}
                _ => {}
            };
            panic!("{}", error);
        }
    };
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
    let time_on_sync_from_master = timespec_from_slice(array_ref![follow_up_message, 4, 12]);
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
        timespec_from_slice(array_ref![delay_reponse_message, 4, 12]);
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
        "[timekeeper] Estimated {} milliseconds of propagation delay. Fixing...",
        propagation_delay.num_milliseconds()
    );

    // Adjust the internal clock with such offset.
    set_time_by_offset(propagation_delay, log.clone());

    Ok(())
}
