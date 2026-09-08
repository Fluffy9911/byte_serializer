
//first 2 bytes define header size

//a header has info about the data stored.
//header has a 8 byte id (utf-8 string), an 8 byte start and 8 byte end

use crate::buffer::Buffer;
use crate::block;
pub fn read_header_size(buf: &mut Buffer) -> Option<u16>{
    buf.set_cursor_pos(0);
   let d =  buf.read_slice(0,1);

   if d.is_some() {

      return  Some(block::combine_u16(<[u8; 2]>::try_from(d.unwrap()).unwrap()));

   }
    None
}


pub fn write_header_size(buf:&mut Buffer,size:u16) {




}



