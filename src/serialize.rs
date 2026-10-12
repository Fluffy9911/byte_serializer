
use crate::buffer::Buffer;
use crate::data::{Data, RawValue};
use crate::{block, schema};

pub trait Serialize {
    fn serialize(&self) -> Option<Data>;
}


pub trait Deserialize {
    fn deserialize(data:&Data)-> Option<Self> where Self: Sized;
}

pub trait ByteSerialize {
    fn serialize(&self) -> Option<Box<[u8]>>;

}

pub trait ByteDeserialize{
    fn deserialize(data:&Buffer) -> Self;
}

impl<T> Serialize for Vec<T> where T: Serialize {
    fn serialize(&self) -> Option<Data> {
let mut list = Data::new();
        let mut d = Data::new();

        for item in self.iter().enumerate() {

            let item_data = item.1.serialize();
            if item_data.is_some() {

            d.write_num_id(item.0 as u64, item_data.unwrap());

            }

        }

list.add_data_unknown("vector".parse().unwrap(), &*d.to_bytes());
        Some(list)

    }
}

pub fn named_vec_serialize<T: Serialize>(name:impl ToString,vec:&Vec<T> )-> Option<Data> {
    let mut list = Data::new();
    let mut d = Data::new();

    for item in vec.iter().enumerate() {

        let item_data = item.1.serialize();
        if item_data.is_some() {

            d.write_num_id(item.0 as u64, item_data.unwrap());

        }

    }

    list.add_data_unknown(name.to_string(), &*d.to_bytes());
    Some(list)
}
pub fn named_vec_deserialize<T: Deserialize>(name:impl ToString,data:Data) -> Option<Vec<T>> {
    let mut vec = Vec::new();
    let list =  data.get_data_unknown(name.to_string());
    let read_data = schema::read_from_buffer(&mut Buffer::new_from_data(list))?;
    for items in read_data.data_map.iter().enumerate() {

        let id = items.0;
        let key: &[u8] = items.1.0;
        let numid = block::read_num_64(key);

        let read_node = schema::read_from_buffer(&mut Buffer::new_from_data(items.1.1));

        if read_node.is_some() {
            let d_t = T::deserialize(read_node.as_ref().unwrap());
            if d_t.is_some() {
                vec.push(d_t.unwrap());
            }
        }

    }

    Some(vec)
}
impl<T> Deserialize for Vec<T> where T: Deserialize {
    fn deserialize(data:&Data) -> Option<Vec<T>> {
let mut vec = Vec::new();
let list =  data.get_data_unknown("vector");
        let read_data = schema::read_from_buffer(&mut Buffer::new_from_data(list))?;
        for items in read_data.data_map.iter().enumerate() {

            let id = items.0;
            let key: &[u8] = items.1.0;
            let numid = block::read_num_64(key);

            let read_node = schema::read_from_buffer(&mut Buffer::new_from_data(items.1.1));

            if read_node.is_some() {
                let d_t = T::deserialize(read_node.as_ref().unwrap());
                if d_t.is_some() {
                    vec.push(d_t.unwrap());
                }
            }

        }

Some(vec)
    }
}

/**

Serializes Data of T into a Data map with Name -> T

*/
pub fn serialize_as_name<T: Serialize>(name:impl ToString,obj: T) -> Option<Data> {

    let mut data_map = Data::new();
   let o_dat = obj.serialize();
    if o_dat.is_some() {
       data_map.add_data_unknown(name.to_string(), &*o_dat.unwrap().to_bytes());
    }
    Some(data_map)
}

impl ByteSerialize for Data {
    fn serialize(&self) -> Option<Box<[u8]>> {
        Some(self.to_bytes())
    }
}
impl ByteDeserialize for Data {
    fn deserialize(data:&Buffer) -> Data {
        schema::read_from_buffer(data).expect("REASON")
    }
}