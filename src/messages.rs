pub enum PTPMessage {
    Sync,
    FollowUp,
    DelayRequest,
    DelayResponse,
}

impl PTPMessage {
    pub fn create_follow_up_message(timestamp: [u8; 8]) -> [u8; 12] {
        let follow_up_magic_number: [u8; 4] = PTPMessage::FollowUp.into();
        let mut message = vec![];
        message.extend_from_slice(&follow_up_magic_number);
        message.extend_from_slice(&timestamp);

        *array_ref!(message, 0, 12)
    }

    pub fn create_delay_response_message(timestamp: [u8; 8]) -> [u8; 12] {
        let delay_response_magic_number: [u8; 4] = PTPMessage::DelayResponse.into();
        let mut message = vec![];
        message.extend_from_slice(&delay_response_magic_number);
        message.extend_from_slice(&timestamp);

        *array_ref!(message, 0, 12)
    }
}

impl From<PTPMessage> for [u8; 4] {
    fn from(message_type: PTPMessage) -> Self {
        match message_type {
            PTPMessage::Sync => [0, 0, 0, 0x00],
            PTPMessage::FollowUp => [0, 0, 0, 0x09],
            PTPMessage::DelayRequest => [0, 0, 0, 0x01],
            PTPMessage::DelayResponse => [0, 0, 0, 0x09],
            _ => panic!("There is no such PTP Message Type"),
        }
    }
}

impl From<[u8; 4]> for PTPMessage {
    #![feature(slice_patterns)]
    fn from(bytes: [u8; 4]) -> Self {
        match &bytes[..] {
            [0, 0, 0, 0x00] => PTPMessage::Sync,
            [0, 0, 0, 0x08] => PTPMessage::FollowUp,
            [0, 0, 0, 0x01] => PTPMessage::DelayRequest,
            [0, 0, 0, 0x09] => PTPMessage::DelayResponse,
            _ => panic!("There is no correspondance between PTP Message Types and message type id {:?}", bytes),
        }
//        if bytes[0] != 0 {
//            panic!("There is no correspondance between PTP Message Types and message type id {:?}", bytes)
//        }
//        if bytes[1] != 0 {
//            panic!("There is no correspondance between PTP Message Types and message type id {:?}", bytes)
//        }
//        if bytes[2] != 0 {
//            panic!("There is no correspondance between PTP Message Types and message type id {:?}", bytes)
//        }
//        match bytes[3] {
//            0x00 => PTPMessage::Sync,
//            0x08 => PTPMessage::FollowUp,
//            0x01 => PTPMessage::DelayRequest,
//            0x09 => PTPMessage::DelayResponse,
//            _ => panic!("There is no correspondance between PTP Message Types and message type id {:?}", bytes),
//        }
    }
}

impl From<u8> for PTPMessage {
    fn from(bytes: u8) -> Self {
        match bytes {
            0x00 => PTPMessage::Sync,
            0x08 => PTPMessage::FollowUp,
            0x01 => PTPMessage::DelayRequest,
            0x09 => PTPMessage::DelayResponse,
            _ => panic!("There is no correspondance between PTP Message Types and message type id {}", bytes),
        }
    }
}