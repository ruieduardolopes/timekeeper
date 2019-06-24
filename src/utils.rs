pub fn from_slice(slice: &[u8; 4]) -> i64 {
    ((slice[0] as i64) << 24)
        + ((slice[1] as i64) << 16)
        + (((slice[2]) as i64) << 8)
        + ((slice[3]) as i64)
}

pub fn from_slice_8(slice: &[u8; 8]) -> i64 {
    ((slice[0] as i64) << 56)
        + ((slice[1] as i64) << 48)
        + ((slice[2] as i64) << 40)
        + ((slice[3] as i64) << 32)
        + ((slice[4] as i64) << 24)
        + ((slice[5] as i64) << 16)
        + ((slice[6] as i64) << 8)
        + ((slice[7]) as i64)
}

pub fn to_slice(integer: i64) -> [u8; 4] {
    [
        ((integer >> 24) & 0xFF) as u8,
        ((integer >> 16) & 0xFF) as u8,
        ((integer >> 8) & 0xFF) as u8,
        ((integer >> 0) & 0xFF) as u8,
    ]
}

pub fn to_slice_8(integer: i64) -> [u8; 8] {
    [
        ((integer >> 56) & 0xFF) as u8,
        ((integer >> 48) & 0xFF) as u8,
        ((integer >> 40) & 0xFF) as u8,
        ((integer >> 32) & 0xFF) as u8,
        ((integer >> 24) & 0xFF) as u8,
        ((integer >> 16) & 0xFF) as u8,
        ((integer >> 8) & 0xFF) as u8,
        ((integer >> 0) & 0xFF) as u8,
    ]
}

