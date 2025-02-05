#[derive(PartialEq, Eq)]
pub struct Palette {
    pub palette: [u8; 256],
}

impl Palette {
    pub fn new(palette: [u8; 256]) -> Self {
        Self { palette }
    }

    pub fn extract_from_bytes(bytes: &[u8]) -> Self {
        let mut palette = [0; 256];

        for (i, x) in bytes.iter().enumerate() {
            palette[i] = *x;
        }

        Self::new(palette)
    }

    pub fn extract_palettes_from_bytes(bytes: &[u8]) -> Vec<Self> {
        bytes.chunks(256).map(Self::extract_from_bytes).collect()
    }

    pub fn transform_vec(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|&value| self.transform(value)).collect()
    }

    pub fn transform(&self, value: u8) -> u8 {
        self.palette[value as usize]
    }

    pub fn get_neutral_palette() -> Self {
        let mut palette = [0; 256];

        for i in 0..256 {
            palette[i as usize] = i as u8;
        }

        Self { palette }
    }

    pub fn combine_palettes(&self, other: &Self) -> Self {
        let mut new_palette = [0; 256];

        for (i, p) in new_palette.iter_mut().enumerate() {
            *p = self.palette[other.palette[i] as usize];
        }

        Self {
            palette: new_palette,
        }
    }

    pub fn combine_multiple_palettes(a_palettes: &[Self], b_palettes: &[Self]) -> Vec<Self> {
        let mut palettes = Vec::new();

        for a_palette in a_palettes {
            for b_palette in b_palettes {
                palettes.push(a_palette.combine_palettes(b_palette));
            }
        }

        palettes
    }
}
