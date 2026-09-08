
//first 2 bytes define header size

//a header has info about the data stored.
//header has a 8 byte id (utf-8 string), an 8 byte start and 8 byte end

use crate::buffer::Buffer;
use crate::block;
pub fn read_header_size(buf: &mut Buffer) -> Option<u16>{
    buf.set_cursor_pos(0);
   let d =  buf.read_slice(0,1);
println!("{:?}",d);
   if d.is_some() {

      return  Some(block::combine_u16(<[u8; 2]>::try_from(d.unwrap()).unwrap()));

   }
    None
}


pub fn write_header_size(buf:&mut Buffer,size:u16) {
   let bytes = block::split_u16(size);
buf.set_cursor_pos(0);
    buf.write_and_extend(&bytes);

}

pub fn read_headers(buf:&mut Buffer,size:u16) -> Option<Vec<[u8;3]>>{
    let mut v: Vec<[u8;3]> = Vec::new();
    for i in 0..size {

        let d = buf.read_slice((i * (8 * 3)) as usize, (i * (8 * 3) + (8 * 3)) as usize);


        if d.is_some() {
            v.push(<[u8; 3]>::try_from(d.unwrap()).unwrap());

        }else{
            panic!("Header Size and data found is mismatched");
          return  None;
        }

    }

    Some(v)

}

pub fn get_data_size(ids:Vec<String>,data: Vec<Box<[u8]>>)-> u64{

    let mut size = 2;

    for str in ids {

        let id_s = str.as_bytes();
        if id_s.len() > 8 {
            size += 8;
        }
        else{
            size += id_s.len() as u64;
        }

    }

    for d in data {

        size += d.len() as u64;

    }

    size

}


pub fn get_headers_size(ids: Vec<String>) -> u64 {
    let mut size = 2;

    for str in ids {

        let id_s = str.as_bytes();
        if id_s.len() > 8 {
            size += 8;
        }
        else{
            size += id_s.len() as u64;
        }

    }
    size


}

pub struct Data {

    ids: Vec<String>,
    data: Vec<Box<[u8]>>,

}

impl Data {


    pub fn new(){

        Data { ids: Vec::new(), data: Vec::new() };

    }

    pub fn add_data(&mut self,id:String, data:Box<[u8]>){
        self.ids.push(id);
        self.data.push(data);
    }




}