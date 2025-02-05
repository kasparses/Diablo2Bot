use crate::{enums::quality::Quality, palette::Palette};

pub struct QualityPalette {
    pub color: Quality,
    pub palette: Palette,
}

pub struct QualityPaletteIndex {
    pub color: Quality,
    pub index: usize,
}
