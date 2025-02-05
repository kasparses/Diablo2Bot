pub struct ByteStream<'bytes> {
    bytes: &'bytes [u8],
    offset: usize,
}

impl<'bytes> ByteStream<'bytes> {
    pub fn new(bytes: &'bytes [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn stream_i16(&mut self) -> i16 {
        i16::from_le_bytes(self.stream_two_bytes())
    }

    pub fn stream_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.stream_four_bytes())
    }

    pub fn stream_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.stream_two_bytes())
    }

    pub fn stream_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.stream_four_bytes())
    }

    pub fn stream_u8(&mut self) -> u8 {
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

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    fn stream_two_bytes(&mut self) -> [u8; 2] {
        let bytes: [u8; 2] = self.bytes[self.offset..self.offset + 2]
            .try_into()
            .expect("slice with incorrect length");

        self.offset += 2;

        bytes
    }

    fn stream_four_bytes(&mut self) -> [u8; 4] {
        let bytes: [u8; 4] = self.bytes[self.offset..self.offset + 4]
            .try_into()
            .expect("slice with incorrect length");

        self.offset += 4;

        bytes
    }
}
