//first 2 bytes define header size

//a header has info about the data stored.
//header has a 8 byte id (utf-8 string), an 8 byte start and 8 byte end

use std::process::id;
use crate::buffer::Buffer;
use crate::{block};
use crate::block::{combine_u16, combine_u32, combine_u64, split_u16, split_u32, split_u64};

pub fn read_header_size(buf: &mut Buffer) -> Option<u16> {
    buf.set_cursor_pos(0);
    let d = buf.read_slice(0, 1);

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

pub fn read_headers(buf: &mut Buffer, size: u16) -> Option<Vec<[u64; 3]>> {
    let mut v = Vec::with_capacity(size as usize);

    const HEADER_SIZE: usize = 24;
    const HEADER_START: usize = 2;

    for i in 0..size as usize {
        let start = HEADER_START + i * HEADER_SIZE;
        let end = start + HEADER_SIZE;

        if end > buf.data.len() {
            return None;
        }

        let d = &buf.data[start..end];





        let values = block::convert_array_u8_64(d);

        v.push(<[u64; 3]>::try_from(values).unwrap());
    }

    Some(v)
}

pub fn get_data_size(ids: &[String], data: &[Box<[u8]>]) -> u64 {
    let header_size = get_headers_size(ids);

    let data_size: usize = data
        .iter()
        .map(|d| d.len())
        .sum();

    (header_size as usize + data_size) as u64
}

pub fn get_headers_size(ids: &[String]) -> u64 {
    (2 + ids.len() * 24) as u64
}

fn string_to_u8_8(s: &str) -> [u8; 8] {
    let mut result = [0u8; 8];

    let bytes = s.as_bytes();
    let len = bytes.len().min(8);

    result[..len].copy_from_slice(&bytes[..len]);

    result
}


#[derive(Debug)]
pub struct Data {
    pub ids: Vec<String>,
    pub data: Vec<Box<[u8]>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            ids: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, id: String, data: Box<[u8]>) {
        self.ids.push(id);
        self.data.push(data);
    }
    pub fn add_data_unknown(&mut self, id: String, data: &[u8]) {
        self.ids.push(id);
        self.data.push(data.to_vec().into_boxed_slice());
    }
    pub fn get_data_unknown(&self, id: impl Into<String>) -> &[u8] {
        let id = id.into();

        let index = self.ids
            .iter()
            .position(|x| x == &id)
            .expect("Data ID not found");

        &self.data[index]
    }
    pub fn write_to_buffer(&self) -> Buffer {
        let mut buf =
            Buffer::new_with_size((get_data_size(&self.ids, &self.data)) as usize);

        let headers = self.ids.len();
        //bad cast
        write_header_size(&mut buf, headers as u16);

        let mut start_off = get_headers_size(&self.ids);
        let mut header_id = 0;

        for i in self.data.iter().enumerate() {
            let id = &self.ids[i.0];

            let id_data = string_to_u8_8(id);

            let dd = i.1;

            let header_off: u64 = header_id * (3 * 8) + 2;

            buf.write_at_u88(header_off as usize, id_data);
            buf.write_at_u88((header_off + 8) as usize, block::split_u64(start_off));


            buf.write_at_u8_arr(start_off as usize,dd.as_ref());
            start_off += (dd.len() as u64);

            buf.write_at_u88((header_off + 16) as usize, block::split_u64(start_off));




            header_id +=1;
        }

        buf
    }

    pub fn put_u8(&mut self, id: impl Into<String>, value: u8) {
        self.add_data_unknown(id.into(), &vec![value].into_boxed_slice());
    }

    pub fn get_u8(&self, id: impl Into<String>) -> Option<u8> {
        self.get_data_unknown(id).first().copied()
    }


    pub fn put_u16(&mut self, id: impl Into<String>, value: u16) {
        self.add_data_unknown(
            id.into(),
            &split_u16(value) ,
        );
    }

    pub fn get_u16(&self, id: impl Into<String>) -> Option<u16> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 2 {
            return None;
        }

        Some(combine_u16(bytes.try_into().ok()?))
    }


    pub fn put_u32(&mut self, id: impl Into<String>, value: u32) {
        self.add_data_unknown(
            id.into(),
            &split_u32(value) ,
        );
    }

    pub fn get_u32(&self, id: impl Into<String>) -> Option<u32> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 4 {
            return None;
        }

        Some(combine_u32(bytes.try_into().ok()?))
    }


    pub fn put_u64(&mut self, id: impl Into<String>, value: u64) {
        self.add_data_unknown(
            id.into(),
           & split_u64(value) ,
        );
    }

    pub fn get_u64(&self, id: impl Into<String>) -> Option<u64> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 8 {
            return None;
        }

        Some(combine_u64(bytes.try_into().ok()?))
    }


    // ------------------------------------------------------------
    // Signed integers
    // ------------------------------------------------------------

    pub fn put_i8(&mut self, id: impl Into<String>, value: i8) {
        self.put_u8(id, value as u8);
    }

    pub fn get_i8(&self, id: impl Into<String>) -> Option<i8> {
        self.get_u8(id).map(|v| v as i8)
    }


    pub fn put_i16(&mut self, id: impl Into<String>, value: i16) {
        self.add_data_unknown(
            id.into(),
            &value.to_be_bytes() ,
        );
    }

    pub fn get_i16(&self, id: impl Into<String>) -> Option<i16> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 2 {
            return None;
        }

        Some(i16::from_be_bytes(bytes.try_into().ok()?))
    }


    pub fn put_i32(&mut self, id: impl Into<String>, value: i32) {
        self.add_data_unknown(
            id.into(),
            &value.to_be_bytes() ,
        );
    }

    pub fn get_i32(&self, id: impl Into<String>) -> Option<i32> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 4 {
            return None;
        }

        Some(i32::from_be_bytes(bytes.try_into().ok()?))
    }


    pub fn put_i64(&mut self, id: impl Into<String>, value: i64) {
        self.add_data_unknown(
            id.into(),
            &value.to_be_bytes() ,
        );
    }

    pub fn get_i64(&self, id: impl Into<String>) -> Option<i64> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 8 {
            return None;
        }

        Some(i64::from_be_bytes(bytes.try_into().ok()?))
    }


    // ------------------------------------------------------------
    // Floating point
    // ------------------------------------------------------------

    pub fn put_f32(&mut self, id: impl Into<String>, value: f32) {
        self.add_data_unknown(
            id.into(),
            &value.to_bits().to_be_bytes(),
        );
    }

    pub fn get_f32(&self, id: impl Into<String>) -> Option<f32> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 4 {
            return None;
        }

        let bits = u32::from_be_bytes(bytes.try_into().ok()?);

        Some(f32::from_bits(bits))
    }


    pub fn put_f64(&mut self, id: impl Into<String>, value: f64) {
        self.add_data_unknown(
            id.into(),
            &value.to_bits().to_be_bytes(),
        );
    }

    pub fn get_f64(&self, id: impl Into<String>) -> Option<f64> {
        let bytes = self.get_data_unknown(id);

        if bytes.len() != 8 {
            return None;
        }

        let bits = u64::from_be_bytes(bytes.try_into().ok()?);

        Some(f64::from_bits(bits))
    }


    // ------------------------------------------------------------
    // Boolean
    // ------------------------------------------------------------

    pub fn put_bool(&mut self, id: impl Into<String>, value: bool) {
        self.put_u8(id, value as u8);
    }

    pub fn get_bool(&self, id: impl Into<String>) -> Option<bool> {
        match self.get_u8(id)? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

}

pub fn read_from_buffer(buf: &mut Buffer) -> Option<Data> {
    let mut data: Data = Data::new();

    let header_dat = read_header_size(buf);

    if header_dat.is_some() {
        let size = header_dat.unwrap();
        let headers = read_headers(buf, size);

        if headers.is_some() {
            for bytes in headers.unwrap() {
                let id_bytes = block::split_u64(bytes[0]);
                let mut id_string = String::from_utf8_lossy(&*id_bytes.to_vec()).to_string();
                //hacky way to remove null characters
                //fix sometime

                id_string = id_string.trim().replace("\0","");
                let start: u64 = bytes[1];
                let end: u64 = bytes[2];

                let bytes_read: &[u8] = &buf.data()[start as usize..end as usize];

                if bytes_read.len() > 0 {
                    let read = bytes_read;
                    data.add_data_unknown(id_string, &read);
                }else{
                    println!("no bytes read");
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

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {

        self.ids == other.ids && self.data == other.data

    }
}


