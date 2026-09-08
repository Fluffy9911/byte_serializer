use byte_serializer::buffer;
use byte_serializer::buffer::Buffer;



fn main() {

let mut buffer: Buffer = Buffer::new_with_size(8);

    for i in 0..7{
        buffer.write(buffer::rand_byte());
        buffer.advance();
    }

    println!("here: {}",buffer.write_to_string());
    buffer.iterate_by(4, |x| {

        println!("block");
        println!("data {:?}",x);

    })

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