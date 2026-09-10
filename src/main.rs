use std::fmt::{Binary, Formatter};
use byte_serializer::{buffer, schema};
use byte_serializer::buffer::Buffer;
use byte_serializer::schema::Data;

fn main() {

let mut data: Data = Data::new();
    let strdat = [0b01101000, 0b01100101, 0b01101100, 0b01101100, 0b01101111];
  data.add_data("test".to_string(),Box::new(strdat));
 let buf =    data.write_to_buffer();

    println!("buffer: {:?}",buf.data());

}
fn split_u16(value: u16) -> (u8, u8) {
    (
        (value >> 8) as u8,
        value as u8,
    )
}

fn combine_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

