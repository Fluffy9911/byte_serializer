use std::array::TryFromSliceError;
use crate::schema::Data;

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


pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn convert_array_u8_64(p0: &[u8]) -> Vec<u64> {
    assert!(
        p0.len() % 8 == 0,
        "Cannot convert {} bytes to u64: length is not divisible by 8",
        p0.len()
    );

    p0.chunks_exact(8)
        .map(|chunk| {
            combine_u64(chunk.try_into().unwrap())
        })
        .collect()
}

pub fn put_str(
    data: &mut Data,
    id: impl Into<String>,
    string: impl Into<String>,
) {
    data.add_data_unknown(
        id.into(),
        &string.into().into_bytes().into_boxed_slice(),
    );
}

pub fn read_str(
    data: &mut Data,
    id: impl Into<String>,
) -> String {
    let id = id.into();

    let bytes = data
        .get_data_unknown(&id);

    String::from_utf8_lossy(bytes).into_owned()
}

