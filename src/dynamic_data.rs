use crate::buffer::Buffer;
use crate::block;
use crate::serialize::{ByteDeserialize, ByteSerialize};

pub struct DynamicData {
    pub size: usize,
    pub data: Buffer,
}

impl DynamicData {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: Buffer::new(),
        }
    }

    pub fn from_data(size: usize, data: &[u8]) -> Self {
        assert!(data.len() <= size, "Data exceeds capacity");

        Self {
            size,
            data: Buffer::new_from_data(data),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.data.data.len() <= self.size
    }
}

impl ByteSerialize for DynamicData {
    fn serialize(&self) -> Option<Box<[u8]>> {
        if !self.is_valid() {
            return None;
        }

        let mut buf = Buffer::new();

        // Store capacity.
        buf.data.extend(block::split_u64(self.size as u64));

        // Store actual data length.
        buf.data.extend(block::split_u64(self.data.data.len() as u64));

        // Store data.
        buf.data.extend_from_slice(&self.data.data);

        // Zero-pad to the specified capacity.
        buf.data.resize(16 + self.size, 0);

        Some(buf.data.into_boxed_slice())
    }
}

impl ByteDeserialize for DynamicData {
    fn deserialize(input: &Buffer) -> Self {
        const HEADER_SIZE: usize = 16;

        assert!(input.data.len() >= HEADER_SIZE);

        let size = u64::from_le_bytes(
            input.data[0..8].try_into().unwrap(),
        ) as usize;

        let data_len = u64::from_le_bytes(
            input.data[8..16].try_into().unwrap(),
        ) as usize;

        assert!(data_len <= size);
        assert_eq!(input.data.len(), HEADER_SIZE + size);

        let data = Buffer::new_from_data(
            &input.data[HEADER_SIZE..HEADER_SIZE + data_len],
        );

        Self { size, data }
    }
}