use crate::messages::*;
use crate::utils::*;
use chrono::offset::TimeZone;
use chrono::Utc;
use slog::Logger;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use slog::{info};

pub fn handle_client(mut stream: TcpStream, log: Logger) -> Result<(), Error> {
    // Send Sync message to slave.
    let sync_message: &[u8; 4] = &to_slice(PTPMessage::Sync as i64);
    info!(log, "[timekeeper] Sending a Sync message to slave...");
    let time_on_sync_sending = time::now().to_timespec();
    info!(
        log,
        "[timekeeper] Marked {}.{} as timestamp on sending Sync message",
        time_on_sync_sending.sec,
        time_on_sync_sending.nsec
    );
    stream.write(sync_message);

    // Send Follow-Up message to slave.
    let follow_up_message = PTPMessage::create_follow_up_message(time_on_sync_sending);
    info!(
        log,
        "[timekeeper] Sending Follow-Up message with {}.{} as timestamp in it",
        time_on_sync_sending.sec,
        time_on_sync_sending.nsec
    );
    stream.write(&follow_up_message);

    // Receive Delay Request from slave.
    let mut delay_request_message: [u8; 4] = [0; 4];
    info!(
        log,
        "[timekeeper] Waiting for a Delay Request message from slave..."
    );
    stream.read(&mut delay_request_message)?;
    let time_on_delay_request = time::now().to_timespec();
    info!(
        log,
        "[timekeeper] Marked {}.{} as timestamp on receiving Delay Request",
        time_on_delay_request.sec,
        time_on_delay_request.nsec
    );

    // Send Delay Response to slave, with the registered value.
    let delay_response_message = PTPMessage::create_delay_response_message(time_on_delay_request);
    info!(
        log,
        "[timekeeper] Sending Delay Response message with {}.{} as timestamp in it",
        time_on_delay_request.sec,
        time_on_delay_request.nsec
    );
    stream.write(&delay_response_message)?;

    Ok(())
}
