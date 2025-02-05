#[derive(Debug)]
pub struct BitStream<'bytes> {
    pub bytes: &'bytes [u8],
    pub byte_offset: usize,
    pub bit_offset: u8, // 0,1,2,3,4,5,6,7
}

impl<'bytes> BitStream<'bytes> {
    pub fn new(bytes: &'bytes [u8]) -> Self {
        Self {
            bytes,
            byte_offset: 0,
            bit_offset: 0,
        }
    }

    pub fn new_with_bit_offset(bytes: &'bytes [u8], bit_offset: u8) -> Self {
        Self {
            bytes,
            byte_offset: 0,
            bit_offset,
        }
    }

    pub fn stream_alligned_uint(&mut self) -> u32 {
        u32::from_le_bytes(self.stream_four_bytes())
    }

    pub fn stream_alligned_byte(&mut self) -> u8 {
        self.validate_byte_read();
        let byte = self.bytes[self.byte_offset];
        self.byte_offset += 1;
        byte
    }

    pub fn stream_byte(&mut self, num_bits: u8) -> u8 {
        let mut byte = 0;

        for i in 0..num_bits {
            if Self::has_bit(self.bytes[self.byte_offset], self.bit_offset) {
                byte |= 1 << i;
            }

            self.increment_bit_offset();
        }

        byte
    }

    pub fn stream_bits(&mut self, num_bits: u8) -> u32 {
        let bytes = self.get_bits_as_bytes(num_bits);
        u32::from_le_bytes(bytes)
    }

    pub fn stream_bit(&mut self) -> bool {
        let result = Self::has_bit(self.bytes[self.byte_offset], self.bit_offset);

        self.increment_bit_offset();

        result
    }

    pub fn stream_signed_bits(&mut self, num_bits: u8) -> i32 {
        if num_bits == 0 {
            return 0;
        }

        if num_bits == 1 {
            if Self::has_bit(self.bytes[self.byte_offset], self.bit_offset) {
                self.increment_bit_offset();
                return -1;
            }
            self.increment_bit_offset();
            return 0;
        }

        let bytes = self.get_bits_as_bytes(num_bits);
        Self::extend_sign(bytes, num_bits)
    }

    fn increment_bit_offset(&mut self) {
        self.bit_offset += 1;
        if self.bit_offset >= 8 {
            self.bit_offset = 0;
            self.byte_offset += 1;
        }
    }

    fn extend_sign(bytes: [u8; 4], bits_number: u8) -> i32 {
        let mut value = i32::from_le_bytes(bytes);

        if value & (1 << (bits_number - 1)) != 0 {
            value |= !((1 << bits_number) - 1);
        }

        value
    }

    fn validate_byte_read(&self) {
        if self.bit_offset != 0 {
            panic!("Attempt to read unalligned byte(s)")
        }
    }

    fn stream_four_bytes(&mut self) -> [u8; 4] {
        self.validate_byte_read();
        let bytes: [u8; 4] = self.bytes[self.byte_offset..self.byte_offset + 4]
            .try_into()
            .expect("slice with incorrect length");
        self.byte_offset += 4;
        bytes
    }

    fn has_bit(byte: u8, bit_position: u8) -> bool {
        byte & (1 << bit_position) != 0
    }

    fn get_bits_as_bytes(&mut self, num_bits: u8) -> [u8; 4] {
        let mut bytes: [u8; 4] = [0; 4];

        if num_bits == 0 {
            return bytes;
        }

        if num_bits > 32 {
            panic!("can't read more than 32 bits")
        }

        let mut dst_bit: u8 = 0;
        let mut dst_byte: u8 = 0;

        for _ in 0..num_bits {
            let byte = self.bytes[self.byte_offset];

            if Self::has_bit(byte, self.bit_offset) {
                bytes[dst_byte as usize] |= 1 << dst_bit;
            }

            dst_bit += 1;
            if dst_bit >= 8 {
                dst_bit = 0;
                dst_byte += 1;
            }

            self.increment_bit_offset();
        }

        bytes
    }
}
