use dcc_decoder::Dcc;

pub struct DccBytes {
    bytes: Vec<u8>,
}

impl DccBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn parse(&self) -> Dcc {
        Dcc::new(&self.bytes)
    }
}
