use crate::palette::Palette;

pub struct RandTransformRawBytes {
    bytes: Vec<u8>,
}

impl RandTransformRawBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn parse(&self) -> RandTransformPalettes {
        RandTransformPalettes::new(&self.bytes)
    }
}

pub struct RandTransformPalettes {
    pub palettes: Vec<Palette>,
}

impl RandTransformPalettes {
    pub fn new(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 7680);

        let palettes = Palette::extract_palettes_from_bytes(bytes);

        Self { palettes }
    }
}
