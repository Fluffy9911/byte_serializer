use crate::block;
use crate::buffer::Buffer;
use crate::serialize::{ByteDeserialize, ByteSerialize};

pub struct DataList {
    block_size: usize,
    blocks: Vec<Buffer>
}

impl DataList {
    pub fn new(block_size: usize) -> DataList {
        DataList {block_size, blocks: Vec::new()}
    }

    pub fn is_valid(&self,buf:&Buffer) -> bool {
        buf.data.len() < self.block_size
    }

    pub fn append_boxed(&mut self, data:Box<[u8]>) {
        let buffer = Self::to_buffer(data);
       self.append_buffer(buffer);
    }
    pub fn append_buffer(&mut self, buffer: Buffer){
        if self.is_valid(&buffer) {
            self.blocks.push(buffer);
        }else{
            println!("The Data Wished to be added is bigger than the currently allowed max: D{}, M{}",buffer.data.len(),self.block_size);
        }

    }
    pub fn to_buffer(data:Box<[u8]>) -> Buffer {
        Buffer::new_from_data(&*data)
    }
}


impl ByteSerialize for DataList {
    fn serialize(&self) -> Option<Box<[u8]>> {
        let mut buf = Buffer::new();

        // Store the block size.
        buf.data.extend(block::split_u64(self.block_size as u64));

        // Store each block in its own fixed-size slot.
        for block in &self.blocks {
            buf.data.extend_from_slice(&block.data);

            buf.data.resize(
                buf.data.len() + self.block_size - block.data.len(),
                0,
            );
        }

        Some(buf.data.into_boxed_slice())
    }
}

impl ByteDeserialize for DataList {
    fn deserialize(data: &Buffer) -> Self {
        const HEADER_SIZE: usize = 8;

        assert!(data.data.len() >= HEADER_SIZE);

        let block_size = u64::from_le_bytes(
            data.data[..HEADER_SIZE].try_into().unwrap(),
        ) as usize;

        assert!(block_size > 0);

        let payload = &data.data[HEADER_SIZE..];

        assert_eq!(
            payload.len() % block_size,
            0,
            "Serialized data contains an incomplete block"
        );

        let blocks = payload
            .chunks_exact(block_size)
            .map(Buffer::new_from_data)
            .collect();

        DataList {
            block_size,
            blocks,
        }
    }
}