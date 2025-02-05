use std::{fs::File, io::BufWriter, path::Path};

use crate::{matrix::Matrix, pal_pl2::PaletteTransformer, point_u16::PointU16, structs::Pixel};

#[derive(PartialEq, Debug)]
pub struct Image {
    pub dims: PointU16,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn to_matrix(&self, palette_transformer: &PaletteTransformer) -> Matrix {
        Matrix::new(self.dims, palette_transformer.contract(&self.pixels))
    }

    pub fn get_diff_percentage(&self, other: &Self) -> f32 {
        assert_eq!(self.pixels.len(), other.pixels.len());

        let mut diff = 0;
        let total = self.pixels.len();

        for (a, b) in self.pixels.iter().zip(other.pixels.iter()) {
            if a != b {
                diff += 1;
            }
        }

        diff as f32 / total as f32
    }

    pub fn get_value(&self, point: PointU16) -> Pixel {
        let idx = self.get_index(point);
        self.pixels[idx]
    }

    pub fn save_image(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        let writer = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, self.dims.col as u32, self.dims.row as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        let mut data = Vec::new();

        for pixel in &self.pixels {
            data.push(pixel.red);
            data.push(pixel.green);
            data.push(pixel.blue);
        }

        writer.write_image_data(&data)?;

        Ok(())
    }

    fn get_index(&self, point: PointU16) -> usize {
        (u32::from(point.row) * u32::from(self.dims.col) + u32::from(point.col)) as usize
    }
}
