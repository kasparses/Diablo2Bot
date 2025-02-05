use dc6_decoder::Dc6;

pub struct Dc6RawBytes {
    bytes: Vec<u8>,
}

impl Dc6RawBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn parse(&self) -> Dc6 {
        Dc6::new(&self.bytes)
    }
}
