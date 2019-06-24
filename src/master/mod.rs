use crate::messages::*;
use crate::utils::*;
use chrono::offset::TimeZone;
use chrono::Utc;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // Send Sync message to slave.
    let sync_message: &[u8; 4] = &to_slice(PTPMessage::Sync as i64);
    let time_on_sync_sending = Utc::now().timestamp_millis();
    stream.write(sync_message);

    // Send Follow-Up message to slave.
    let follow_up_message = PTPMessage::create_follow_up_message(to_slice_8(time_on_sync_sending));
    stream.write(&follow_up_message);

    // Receive Delay Request from slave.
    let mut delay_request_message: [u8; 4] = [0; 4];
    stream.read(&mut delay_request_message)?;
    let time_on_delay_request = Utc::now().timestamp_millis();

    // Send Delay Response to slave, with the registered value.
    let delay_response_message = PTPMessage::create_delay_response_message(to_slice_8(time_on_delay_request));
    stream.write(&delay_response_message)?;

    Ok(())
}
