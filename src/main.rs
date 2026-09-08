use byte_serializer::{buffer, schema};
use byte_serializer::buffer::Buffer;



fn main() {

let mut buffer: Buffer = Buffer::new_with_size(8);

    schema::write_header_size(&mut buffer, 2);
let d = &buffer.data()[0..=1];
    println!("{:?}",schema::read_header_size(&mut buffer));

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