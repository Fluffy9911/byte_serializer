//first 2 bytes define header size

//a header has info about the data stored.
//header has a 8 byte id (utf-8 string), an 8 byte start and 8 byte end

use std::collections::HashMap;
use crate::buffer::Buffer;
use crate::{block, schema};
use crate::data::Data;


pub const header_size_bytes: usize = 2;
pub const header_id_size_bytes: usize = 64;
pub const header_id_start_size: usize = 8;
pub const header_id_end_size: usize = 8;
pub fn read_header_size(buf: & Buffer) -> Option<u16> {

    let d = buf.read_slice(0, header_size_bytes-1);
    if d.is_some() {
        return Some(block::combine_u16(<[u8; 2]>::try_from(d.unwrap()).unwrap()));
    }
    None
}

pub fn write_header_size(buf: &mut Buffer, size: u16) {
    let bytes = block::split_u16(size);
    buf.set_cursor_pos(0);
    buf.write_and_extend(&bytes);
}

pub fn read_headers(buf: &Buffer, size: u16) -> Option<Vec<&[u8]>> {
    let mut v = Vec::with_capacity(size as usize);

    const HEADER_SIZE: usize = schema::header_id_size_bytes + 2*(schema::header_id_start_size);
    const HEADER_START: usize = 2;

    for i in 0..size as usize {
        let start = HEADER_START + i * HEADER_SIZE;
        let end = start + HEADER_SIZE;

        if end > buf.data.len() {
            return None;
        }

        let d = &buf.data[start..end];


        v.push(d);
    }

    Some(v)
}

pub fn get_data_size(map:&HashMap<Box<[u8]>,Box<[u8]>>) -> u64 {
    let header_size = get_headers_size(map.len() as u64, (header_id_size_bytes + header_id_start_size + header_id_end_size) as u64);

    let data_size: usize = map
        .iter()
        .map(|d| d.1.len())
        .sum();

    (header_size as usize + data_size) as u64
}

pub fn get_headers_size(id_count:u64,size:u64) -> u64 {
    (2 + id_count * size) as u64
}

pub fn string_to_u8_8(s: &str) -> [u8; 8] {
    let mut result = [0u8; 8];

    let bytes = s.as_bytes();
    let len = bytes.len().min(8);

    result[..len].copy_from_slice(&bytes[..len]);

    result
}


pub fn read_from_buffer(buf: &Buffer) -> Option<Data> {
    let mut data: Data = Data::new();

    let header_dat = read_header_size(buf);

    if header_dat.is_some() {
        let size = header_dat.unwrap();
        let headers = read_headers(buf, size);

        if headers.is_some() {
            for bytes in headers.unwrap() {

                let header_id = &bytes[0..schema::header_id_size_bytes];

               // let mut id_string = String::from_utf8_lossy(header_id).to_string();

let start_bytes = &bytes[schema::header_id_size_bytes as usize..schema::header_id_size_bytes as usize+schema::header_id_start_size as usize];
         let end_bytes = &bytes[schema::header_id_size_bytes as usize+schema::header_id_end_size as usize..];
                let start: u64 = block::combine_u64(<[u8;8]>::try_from(start_bytes).unwrap());
                let end: u64 = block::combine_u64(<[u8;8]>::try_from(end_bytes).unwrap());

                let bytes_read: &[u8] = &buf.data()[start as usize..end as usize];

                if bytes_read.len() > 0 {
                    let read = bytes_read;
                    data.add_data_unknown_bytes(header_id, &read);
                }
            }
           return Some(data);
        }else{
            println!("no headers");
        }
    }
    panic!("Malformed Buffer Data");
    None
}


