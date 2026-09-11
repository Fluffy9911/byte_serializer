use std::fmt::{Binary, Formatter};
use byte_serializer::{buffer, schema};
use byte_serializer::block::put_str;
use byte_serializer::buffer::Buffer;
use byte_serializer::schema::Data;

fn main() {

let mut data: Data = Data::new();
  
    
    data.put_bool("bool",true);
    
 let mut buf =    data.write_to_buffer();


   let read =  schema::read_from_buffer(&mut buf).unwrap();

    assert_eq!(data,read);
}


