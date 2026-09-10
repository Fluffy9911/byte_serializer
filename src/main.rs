use std::fmt::{Binary, Formatter};
use byte_serializer::{buffer, schema};
use byte_serializer::buffer::Buffer;
use byte_serializer::schema::Data;

fn main() {

let mut data: Data = Data::new();
    let strdat = [0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111];
  data.add_data("test".to_string(),Box::new(strdat));

    put_str(&mut data,"hellowworld","Hello World");

 let mut buf =    data.write_to_buffer();
    println!("length: {}",buf.data().len());
println!("{:?}",strdat);
    println!("buffer: {:?}",buf.data());

   let read =  schema::read_from_buffer(&mut buf).unwrap();

    assert_eq!(data,read);
}

pub fn put_str(data: &mut Data,id:impl Into<String>,str: impl Into<String>) {

    //let str_dat: &[u8] = str.into().as_bytes();

    data.add_data_unknown(id.into(), str.into().as_bytes());

}

