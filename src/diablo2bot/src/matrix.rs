use dc6_decoder::{DecodedFrame, EncodedFrame};

use crate::{
    box_u16::BoxU16, image::Image, pal_pl2::PixelPalette, palette::Palette, point_u16::PointU16,
    structs::PointValue,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Matrix {
    pub dims: PointU16,
    pub data: Vec<u8>,
}

impl Matrix {
    pub fn new(dims: PointU16, data: Vec<u8>) -> Self {
        assert_eq!(u32::from(dims.row) * u32::from(dims.col), data.len() as u32);
        Self { dims, data }
    }

    pub fn remove_last_row(&mut self) {
        if self.dims.row == 0 {
            return;
        }

        self.dims.row -= 1;

        let new_len = self.data.len() - self.dims.col as usize;
        self.data.truncate(new_len);
    }

    pub fn new_empty(dims: PointU16) -> Self {
        Self {
            dims,
            data: vec![0; dims.len() as usize],
        }
    }

    pub fn from_dc6_encoded_frame(encoded_frame: &EncodedFrame) -> Self {
        let decoded_frame = encoded_frame.decode();

        Self::new(
            PointU16 {
                row: decoded_frame.meta_data.height as u16,
                col: decoded_frame.meta_data.width as u16,
            },
            decoded_frame.decoded_bytes,
        )
    }

    pub fn from_dc6_decoded_frame(decoded_frame: DecodedFrame) -> Self {
        Self::new(
            PointU16 {
                row: decoded_frame.meta_data.height as u16,
                col: decoded_frame.meta_data.width as u16,
            },
            decoded_frame.decoded_bytes,
        )
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set_value(&mut self, point: PointU16, value: u8) {
        let idx = self.get_index(point);
        self.data[idx] = value;
    }

    pub fn get_value(&self, point: PointU16) -> u8 {
        let idx = self.get_index(point);
        self.data[idx]
    }

    pub fn palette_transform(&self, palette: &Palette) -> Self {
        Self {
            dims: self.dims,
            data: palette.transform_vec(&self.data),
        }
    }

    pub fn get_non_zero_width(&self) -> u16 {
        for col in (0..self.dims.col).rev() {
            if self.col_has_non_zero_value(col) {
                return col + 1;
            }
        }

        0
    }

    fn col_has_non_zero_value(&self, col: u16) -> bool {
        for row in 0..self.dims.row {
            if self.get_value(PointU16 { row, col }) != 0 {
                return true;
            }
        }

        false
    }

    pub fn get_window_points(&self, window: PointU16) -> Vec<PointU16> {
        self.get_box()
            .get_window_box(window)
            .iter_box_points()
            .collect()
    }

    pub fn get_window_offset_points(&self, window: PointU16) -> Vec<PointU16> {
        let mut points = Vec::new();

        for row in 0..=self.dims.row - window.row {
            for col in 0..=self.dims.col - window.col {
                points.push(PointU16 { row, col })
            }
        }

        points
    }

    pub fn get_non_zero_point_values(&self) -> Vec<PointValue> {
        self.iter_points()
            .filter_map(|point| {
                let value = self.get_value(point);
                (value != 0).then_some(PointValue { point, value })
            })
            .collect()
    }

    pub fn get_non_zero_points(&self) -> Vec<PointU16> {
        self.iter_points()
            .filter(|&point| self.get_value(point) != 0)
            .collect()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = PointU16> {
        self.get_box().iter_box_points()
    }

    fn get_box(&self) -> BoxU16 {
        self.dims.into()
    }

    pub fn get_sub_matrix(&self, box_: BoxU16) -> Self {
        let mut data = Vec::new();
        box_.iter_box_points().for_each(|point| {
            data.push(self.get_value(point));
        });

        Self {
            dims: box_.get_dimensions(),
            data,
        }
    }

    pub fn get_diff_percentage(&self, other: &Self) -> f32 {
        assert_eq!(self.data.len(), other.data.len());

        let mut diff = 0;
        let total = self.len();

        for (a, b) in self.data.iter().zip(other.data.iter()) {
            if a != b {
                diff += 1;
            }
        }

        diff as f32 / total as f32
    }

    pub fn get_sub_matrix2(&self, area: BoxU16) -> Self {
        let mut data = Vec::new();
        for row in area.offset.row..area.offset.row + area.dimensions.row {
            for col in area.offset.col..area.offset.col + area.dimensions.col {
                data.push(self.get_value(PointU16 { row, col }));
            }
        }

        Self {
            dims: area.dimensions,
            data,
        }
    }

    pub fn clear_areas(&mut self, areas: &[BoxU16]) {
        for area in areas {
            self.clear_area(*area);
        }
    }

    pub fn clear_area(&mut self, area: BoxU16) {
        for row in area.offset.row..area.offset.row + area.dimensions.row {
            for col in area.offset.col..area.offset.col + area.dimensions.col {
                self.set_value(PointU16 { row, col }, 0);
            }
        }
    }

    pub fn to_image(&self, pixel_palette: &PixelPalette) -> Image {
        let pixels = pixel_palette.expand(&self.data);
        Image {
            dims: self.dims,
            pixels,
        }
    }

    fn insert_row_slice(&mut self, offset: PointU16, row_slice: &[u8]) {
        let idx_offset = self.get_index(offset);

        for (i, val) in row_slice.iter().enumerate() {
            self.data[idx_offset + i] = *val;
        }
    }

    pub fn get_row(&self, row: u16) -> &[u8] {
        let idx = self.get_index(PointU16 { row, col: 0 });
        &self.data[idx..idx + self.dims.col as usize]
    }

    pub fn insert_sub_matrix(&mut self, offset: PointU16, sub_matrix: &Matrix) {
        for row in 0..sub_matrix.dims.row {
            self.insert_row_slice(offset + PointU16 { row, col: 0 }, sub_matrix.get_row(row));
        }
    }

    pub fn get_window(&self, offset: PointU16) -> [u8; 16] {
        let mut window = [0; 16];

        let mut c = 0;
        for row in offset.row..offset.row + 4 {
            for col in offset.col..offset.col + 4 {
                window[c] = self.get_value(PointU16 { row, col });
                c += 1;
            }
        }

        window
    }

    fn get_index(&self, point: PointU16) -> usize {
        (u32::from(point.row) * u32::from(self.dims.col) + u32::from(point.col)) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::point_u16::PointU16;

    use super::Matrix;

    #[test]
    fn test_get_non_zero_width() {
        let dims = PointU16 { row: 8, col: 8 };
        let mut matrix = Matrix::new_empty(dims);
        let col = 5;
        matrix.set_value(
            PointU16 {
                row: 2,
                col: col - 2,
            },
            1,
        );
        matrix.set_value(
            PointU16 {
                row: 6,
                col: col - 1,
            },
            1,
        );
        matrix.set_value(PointU16 { row: 4, col }, 1);

        let non_zero_width = matrix.get_non_zero_width();

        assert_eq!(non_zero_width, col + 1);
    }
}
