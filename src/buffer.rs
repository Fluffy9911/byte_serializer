use std::fmt::format;
use std::string::FromUtf8Error;
use rand::{prelude, Rng, TryRng};
use rand::rngs::ThreadRng;


pub struct Buffer {

    pub(crate) data: Vec<u8>,
cursor: usize
}


impl Buffer {
    pub  fn write_at_u88(&mut self, p0: usize, p1: [u8; 8]) {

        for i in 0..p1.len(){
            self.data[i+p0] = p1[i];

        }

    }
    pub fn new()-> Buffer {
    Self::new_with_size(16)
    }

    pub fn new_with_size(size: usize) -> Buffer {
        Buffer {
            data: vec![0; size],
            cursor: 0,
        }
    }

    pub fn write(&mut self, byte: u8){

        if self.data.len() <= self.cursor{
            self.cursor = self.data.len() - 1;
            self.data[self.cursor] = byte;
        }else{

            self.data[self.cursor] = byte;

        }

    }
pub fn read(&self)-> u8 {
    if self.data.len() < self.cursor{
     0
    } else{
        self.data[self.cursor]
    }
}

    pub fn advance(&mut self){
        self.cursor+=1;
        self.constrain_cursor();

    }

    pub fn constrain_cursor(&mut self){
      self.cursor = self.cursor.min(self.data.len());
    }

    pub fn read_slice(&self,start:usize,end:usize) -> Option< &[u8]>{

        if end < self.data.len(){
         return   Some( &self.data[start..=end])
        }
None}
pub fn write_at_u8_arr(&mut self,pos:usize,data:&[u8]){

    for i in 0..data.len() {

        self.data[i + pos] = data[i];

    }

}

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn expand(&mut self, add:usize){
        self.data.reserve(add);
    }

pub fn write_and_extend(&mut self,data:&[u8]){

    if self.cursor + data.len() > self.data.len(){
        self.expand((self.cursor+ data.iter().len()) - self.data.len());
    }
    for byte in data {
        self.write(*byte);
        self.advance();
    }


}

    pub fn set_cursor_pos(&mut self,pos:usize){
        self.cursor = pos;
        self.constrain_cursor();
    }

    pub fn reverse_buffer(&mut self){
        self.data.reverse();
    }

    pub fn write_to_string(&self) -> String {
        format!("[cursor = {},data = {:?}]",self.cursor,self.data)
    }

    pub fn write_as_string(&self) -> Result<String, FromUtf8Error> {

        String::from_utf8(self.data.clone())

    }

    pub fn iterate_by(&self ,bytes:usize,consumer: impl Fn(&[u8])){

        if self.data.len() % bytes != 0{

        }else{

            for i in 0..(self.data.len()/ bytes) {

                let d = &self.data[i*bytes..(i*bytes+bytes)];

                consumer(d);

            }

        }

    }

}

pub fn rand_byte() -> u8 {
let b = rand::random_range(0..255);
b
}