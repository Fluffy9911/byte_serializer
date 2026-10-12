use std::collections::HashMap;
use std::process::id;
use std::vec;
use crate::buffer::Buffer;
use crate::{block, buffer, schema};
use crate::block::{combine_u16, combine_u32, combine_u64, split_u16, split_u32, split_u64};
use crate::schema::{header_id_end_size, header_id_size_bytes, header_id_start_size};
use crate::serialize::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Data {
    pub data_map: HashMap<Box<[u8]>, Box<[u8]>>,
}



impl Data {
    pub fn new() -> Data {
        Data {
          data_map: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, id: String, data: Box<[u8]>) {
      let mut bytes =   Vec::new();
        let string_bytes = id.as_bytes();
        bytes.extend_from_slice(string_bytes);
        if bytes.len() < 64 {
            //pad so bytes.len = 64

            let diff = 64 - bytes.len();

            for i in 0..diff {
                bytes.push(0);
            }

        }
        self.data_map.insert(Box::from(bytes), data);
    }
    pub fn add_data_unknown(&mut self, id: String, data: &[u8]) {

       self.add_data(id, Box::from(data));
    }
    pub fn add_data_unknown_bytes(&mut self, p0: &[u8], p1: &[u8]) {
        self.data_map.insert(p0.into(), p1.into());
    }
    pub fn get_data_unknown(&self, id: impl Into<String>) -> &[u8] {
        let id = id.into();
        let mut bytes =   Vec::new();
        let string_bytes = id.as_bytes();
        bytes.extend_from_slice(string_bytes);
        if bytes.len() < 64 {
            //pad so bytes.len = 64

            let diff = 64 - bytes.len();

            for i in 0..diff {
                bytes.push(0);
            }
        }

       let found =  self.data_map.get(&Vec::into_boxed_slice(bytes));

        if found.is_some() {
            found.unwrap()
        }else
        {
            panic!("unknown ID");
        }

    }
    pub fn write_to_buffer(&self) -> Buffer {
        let mut buffer = Buffer::new_with_size((schema::get_data_size(&self.data_map)) as usize);

        let headers_count = self.data_map.len();



        schema::write_header_size(&mut buffer, headers_count as u16);

        let mut start_off = schema::get_headers_size(self.data_map.len() as u64,(header_id_size_bytes + header_id_start_size + header_id_end_size) as u64);

        let mut header_id = 0;

        for i in self.data_map.iter() {

            let id = i.0;


            let dd = i.1;

            let header_off: u64 = header_id * (3 * 8) + 2;

buffer.write_arr(id, header_off as usize, schema::header_id_size_bytes, 0);
            buffer.write_at_u88((header_off + (schema::header_id_size_bytes as u64)) as usize, split_u64(start_off));


            buffer.write_at_u8_arr(start_off as usize, dd.as_ref());

            start_off += (dd.len() as u64);

            buffer.write_at_u88((header_off +  (schema::header_id_size_bytes as u64 + 8)) as usize, split_u64(start_off));




            header_id +=1;
        }

        buffer
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

    pub fn get_u64(&self, id: impl Into<String>) -> u64 {
        let bytes = self.get_data_unknown(id);
if bytes.len() != 8 {
    return 0;
}

        
     block::combine_u64(<[u8;8]>::try_from(bytes).ok().unwrap())
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
pub fn iterate(&self,consumer:impl  Fn(&String,&[u8])){

    for item in self.data_map.iter().enumerate(){

        consumer(&String::from_utf8_lossy(item.1.0).as_ref().to_string(), item.1.1.as_ref());

    }

}
    pub fn write_raw(&mut self, id: impl Into<String>, data: Box<dyn RawValue>) {
        self.add_data_unknown(id.into(), &*data.to_bytes())
    }

    pub  fn read_raw<T>(&mut self, id: impl Into<String>, mut from: impl FnMut(&[u8])-> T) -> T {

        let bytes = self.get_data_unknown(id);

        let val: T = from(bytes);
        val
    }

    pub fn write_num_id(&mut self,id: u64,data: impl RawValue) {
        self.data_map.insert(Box::from(block::split_u64(id)), data.to_bytes());
    }

    pub fn read_num_id(&mut self, id: u64) -> Option<Box<[u8]>> {

        let search = self.data_map.get(&Box::from(block::split_u64(id)));
        if let Some(bytes) = search {
            return Some(*Box::new(bytes.clone()));
        }
        None
    }

}



impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {

        self.data_map == other.data_map

    }
}

pub trait RawValue {

     fn to_bytes(&self) -> Box<[u8]>;


}

impl RawValue for Data {
    fn to_bytes(&self) -> Box<[u8]> {
       let data = self.write_to_buffer();

        let bytes = data.data.clone();

       bytes.into_boxed_slice()


    }


}

pub fn from_bytes(bytes: &[u8]) -> Data {

    let mut buf = Buffer::new_from_data(bytes);

    schema::read_from_buffer(&mut buf).unwrap()

}

