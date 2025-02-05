use dc6_decoder::DecodedFrame;

use crate::{matrix::Matrix, point_u16::PointU16};

pub struct Dc6Scene {
    pub top_left: DecodedFrame,
    pub top_right: DecodedFrame,
    pub bottom_left: DecodedFrame,
    pub bottom_right: DecodedFrame,
}

impl Dc6Scene {
    pub fn convert_to_matrix(self) -> Matrix {
        let dims = PointU16 {
            row: (self.top_left.meta_data.height + self.bottom_left.meta_data.height) as u16,
            col: (self.top_left.meta_data.width + self.top_right.meta_data.width) as u16,
        };

        let mut matrix = Matrix::new_empty(dims);

        matrix.insert_sub_matrix(
            PointU16 {
                row: 0,
                col: self.top_left.meta_data.width as u16,
            },
            &Matrix::from_dc6_decoded_frame(self.top_right),
        );

        matrix.insert_sub_matrix(
            PointU16 {
                row: self.top_left.meta_data.height as u16,
                col: 0,
            },
            &Matrix::from_dc6_decoded_frame(self.bottom_left),
        );

        matrix.insert_sub_matrix(
            PointU16 {
                row: self.top_left.meta_data.height as u16,
                col: self.top_left.meta_data.width as u16,
            },
            &Matrix::from_dc6_decoded_frame(self.bottom_right),
        );

        matrix.insert_sub_matrix(
            PointU16 { row: 0, col: 0 },
            &Matrix::from_dc6_decoded_frame(self.top_left),
        );

        matrix
    }
}
