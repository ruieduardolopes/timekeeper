pub fn from_slice(slice: &[u8; 4]) -> i64 {
    ((slice[0] as i64) << 24)
        + ((slice[1] as i64) << 16)
        + (((slice[2]) as i64) << 8)
        + ((slice[3]) as i64)
}

pub fn to_slice(integer: i64) -> [u8; 4] {
    [
        ((integer >> 24) & 0xFF) as u8,
        ((integer >> 16) & 0xFF) as u8,
        ((integer >> 8) & 0xFF) as u8,
        ((integer >> 0) & 0xFF) as u8,
    ]
}
