//first 2 bytes define header size

//a header has info about the data stored.
//header has a 8 byte id (utf-8 string), an 8 byte start and 8 byte end

use std::process::id;
use crate::buffer::Buffer;
use crate::block;
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
    let mut v: Vec<[u64; 3]> = Vec::new();
    for i in 0..size {
        let d = &buf.data[
            (2 + (i * (8 * 3)) as usize)..
                ( 2 + (i * (8 * 3) + (8 * 3)) as usize)
        ];
//println!("{},{},{}",2 + (i * (8 * 3)),2 + (i * (8 * 3) + (8 * 3)),(2 + (i * (8 * 3) + (8 * 3)))+(2 + (i * (8 * 3))));
//println!("{}",d.len());
            v.push(<[u64; 3]>::try_from(block::convert_array_u8_64(d)).unwrap());
        // } else {
        //     panic!("Header Size and data found is mismatched, {},{},",(i * (8 * 3)), (i * (8 * 3)) + (8 * 3));
        //     return None;
        // }
    }

    Some(v)
}

pub fn get_data_size(ids: Vec<String>, data: Vec<Box<[u8]>>) -> u64 {
    let mut size = 2 + (16);

    size += ids.len() * 8;

    for d in data {
        size += d.len();
    }

    size as u64
}

pub fn get_headers_size(ids: &Vec<String>) -> u64 {
    let mut size = 2 +(16);

    for str in ids {
        let id_s = str.as_bytes();
        if id_s.len() > 8 {
            size += 8;
        } else {
            size += 8 as u64;
        }
    }
    size
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
    pub fn write_to_buffer(&self) -> Buffer {
        let mut buf =
            Buffer::new_with_size(get_data_size(self.ids.clone(), self.data.clone()) as usize);

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
            println!("header_begin: {}, start_off: {}, end_off: {}, size {}", header_off, header_off + 8,header_off+16,dd.len());
            buf.write_at_u88(header_off as usize, id_data);
            buf.write_at_u88((header_off + 8) as usize, block::split_u64(start_off));
            buf.write_at_u8_arr(start_off as usize,dd.as_ref()); println!(
                "id: {}, header_off: {}Bytes, start_off: {}",
                id, header_off, start_off
            );
            start_off += (dd.len() as u64);
            println!("endoff {}",start_off);
            buf.write_at_u88((header_off + 16) as usize, block::split_u64(start_off));




            header_id +=1;
        }

        buf
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
println!("id_string: {}, start: {}, end: {}", id_string, start, end);

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


