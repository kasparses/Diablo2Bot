// https://d2mods.info/forum/viewtopic.php?t=30754

use crate::{
    enums::quality::Quality,
    palette::Palette,
    quality_palette::{QualityPalette, QualityPaletteIndex},
    structs::Pixel,
};

const NUM_ACT_PALETTE_BYTES: usize = 256 * 4;
const NUM_FONT_QUALITY_PALETTE_BYTES: usize = 256 * 10;
const FONT_QUALITY_PALETTE_BYTES_OFFSET: usize = 439847;

pub struct PalPl2Bytes {
    bytes: Vec<u8>,
}

impl PalPl2Bytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn extract_act_palette_bytes(&self) -> ActPaletteBytes {
        ActPaletteBytes {
            bytes: self.bytes[0..NUM_ACT_PALETTE_BYTES].to_vec(),
        }
    }

    pub fn extract_font_quality_palette_bytes(&self) -> FontQualityPaletteBytes {
        FontQualityPaletteBytes {
            bytes: self.bytes[FONT_QUALITY_PALETTE_BYTES_OFFSET
                ..FONT_QUALITY_PALETTE_BYTES_OFFSET + NUM_FONT_QUALITY_PALETTE_BYTES]
                .to_vec(),
        }
    }

    pub fn extract_light_radius_palette_bytes(&self) -> LightRadiusPaletteBytes {
        LightRadiusPaletteBytes {
            bytes: self.bytes[20 * 256..34 * 256].to_vec(),
        }
    }
}

pub struct LightRadiusPaletteBytes {
    bytes: Vec<u8>,
}

impl LightRadiusPaletteBytes {
    pub fn get_palettes(&self) -> Vec<Palette> {
        let mut palettes = Vec::new();

        for chunk in self.bytes.chunks(256) {
            palettes.push(Palette::extract_from_bytes(chunk));
        }

        palettes
    }
}

pub struct FontQualityPaletteBytes {
    bytes: Vec<u8>,
}

impl FontQualityPaletteBytes {
    pub fn get_palettes(&self) -> Vec<QualityPalette> {
        let quality_palette_indices = [
            QualityPaletteIndex {
                index: 2,
                color: Quality::Set,
            },
            QualityPaletteIndex {
                index: 3,
                color: Quality::Magic,
            },
            QualityPaletteIndex {
                index: 4,
                color: Quality::Unique,
            },
            QualityPaletteIndex {
                index: 5,
                color: Quality::Grey,
            },
            QualityPaletteIndex {
                index: 8,
                color: Quality::Rune,
            },
            QualityPaletteIndex {
                index: 9,
                color: Quality::Rare,
            },
        ];

        let mut palettes: Vec<QualityPalette> = Vec::new();

        let neutral_palette = Palette::get_neutral_palette();
        palettes.push(QualityPalette {
            color: Quality::Common,
            palette: neutral_palette,
        });

        for quality_palette_index in quality_palette_indices {
            let mut palette = [0; 256];

            let start = quality_palette_index.index * 256;
            let end = start + 256;

            for (i, idx) in (start..end).enumerate() {
                palette[i] = self.bytes[idx];
            }

            palettes.push(QualityPalette {
                color: quality_palette_index.color,
                palette: Palette::new(palette),
            });
        }

        palettes
    }
}

pub struct ActPaletteBytes {
    bytes: Vec<u8>,
}

impl ActPaletteBytes {
    pub fn get_palette_transformer(&self, pixel_palette: &PixelPalette) -> PaletteTransformer {
        let palette_contractor = pixel_palette.get_palette_contractor();

        PaletteTransformer {
            contractor: palette_contractor,
        }
    }

    pub fn get_pixel_palette(&self) -> PixelPalette {
        let mut pixels = [Pixel::default(); 256];

        for (i, p) in pixels.iter_mut().enumerate() {
            let offset = i * 4;
            *p = Self::get_pixel(&self.bytes[offset..])
        }

        PixelPalette { pixels }
    }

    fn get_pixel(bytes: &[u8]) -> Pixel {
        Pixel {
            red: bytes[0],
            green: bytes[1],
            blue: bytes[2],
        }
    }
}

pub struct PaletteTransformer {
    contractor: PaletteContractor,
}

impl PaletteTransformer {
    pub fn contract(&self, data: &[Pixel]) -> Vec<u8> {
        self.contractor.contract(data)
    }
}

struct PaletteContractor {
    transformation_array: Vec<u8>,
}

pub struct PixelPalette {
    pub pixels: [Pixel; 256],
}

impl PixelPalette {
    fn get_palette_contractor(&self) -> PaletteContractor {
        let size = 64 * 64 * 64;
        let mut transformation_array = vec![0; size];

        self.pixels.iter().enumerate().for_each(|(i, pixel)| {
            if pixel.red == 255 || pixel.green == 255 || pixel.blue == 255 {
                if pixel.red != 255 || pixel.green != 255 || pixel.blue != 255 {
                    panic!("Incorrect pixel in act_palette")
                }
            } else {
                transformation_array[pixel.get_flat_index()] = i as u8;
            }
        });

        PaletteContractor {
            transformation_array,
        }
    }

    pub fn expand(&self, data: &[u8]) -> Vec<Pixel> {
        data.iter().map(|value| self._expand(*value)).collect()
    }

    fn _expand(&self, value: u8) -> Pixel {
        self.pixels[value as usize]
    }
}

impl PaletteContractor {
    pub fn contract(&self, data: &[Pixel]) -> Vec<u8> {
        data.iter().map(|&value| self._contract(value)).collect()
    }

    fn _contract(&self, value: Pixel) -> u8 {
        if value.red == 255 {
            return 255;
        }

        self.transformation_array[value.get_flat_index()]
    }
}
