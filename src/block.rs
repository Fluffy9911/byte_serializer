



pub fn split_u16(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

pub fn split_u32(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

pub fn split_u64(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

pub fn combine_u16(v: [u8; 2]) -> u16 {
    u16::from_be_bytes(v)
}

pub fn combine_u32(v: [u8; 4]) -> u32 {
    u32::from_be_bytes(v)
}

pub fn combine_u64(v: [u8; 8]) -> u64 {
    u64::from_be_bytes(v)
}