use std::collections::HashMap;
use std::fmt::{Binary, Formatter};
use std::vec;
use byte_serializer::{block, buffer, debug, schema, serialize};
use byte_serializer::block::put_str;
use byte_serializer::buffer::Buffer;
use byte_serializer::data::{Data, RawValue};
use byte_serializer::serialize::{Deserialize, Serialize};

fn main() {

let mut data = Data::new();

    data.put_u64("Hello",256);

    println!("{:?}",data.get_u64("Hello"));

    assert_eq!(data, schema::read_from_buffer(&mut data.write_to_buffer()).expect("REASON"))

}


#[derive(Debug)]
pub struct Test{

    pub name:String,
    pub age:u32,

}

impl Test {
    pub fn new(name: impl Into<String>,age:u32) -> Test {
        Test { name:name.into(), age:age }
    }
}



