pub struct ByteStream<'bytes> {
    bytes: &'bytes [u8],
    offset: usize,
}

impl<'bytes> ByteStream<'bytes> {
    pub fn new(bytes: &'bytes [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn stream_four_bytes(&mut self) -> [u8; 4] {
        let bytes: [u8; 4] = self.bytes[self.offset..self.offset + 4]
            .try_into()
            .expect("slice with incorrect length");
        self.offset += 4;
        bytes
    }

    pub fn stream_uint(&mut self) -> u32 {
        u32::from_le_bytes(self.stream_four_bytes())
    }

    pub fn stream_int(&mut self) -> i32 {
        i32::from_le_bytes(self.stream_four_bytes())
    }

    pub fn stream_byte(&mut self) -> u8 {
        let byte = self.bytes[self.offset];
        self.offset += 1;
        byte
    }

    pub fn skip_bytes(&mut self, num_bytes: u32) {
        self.offset += num_bytes as usize;
    }

    pub fn stream_bytes(&mut self, num_bytes: u32) -> &'bytes [u8] {
        let byte_slice = &self.bytes[self.offset..self.offset + num_bytes as usize];
        self.offset += num_bytes as usize;
        byte_slice
    }
}
