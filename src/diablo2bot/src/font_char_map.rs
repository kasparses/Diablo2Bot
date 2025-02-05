use std::collections::HashMap;

use crate::matrix::Matrix;

pub fn get_non_control_ascii_char_font_map(dc6_file: &dc6_decoder::Dc6) -> HashMap<char, Matrix> {
    let mut map = HashMap::new();

    for i in 0..=127_u8 {
        let c = i as char;
        if !c.is_control() {
            let matrix = get_frame_matrix(dc6_file, i as usize);
            map.insert(c, matrix);
        }
    }

    map
}

fn get_frame_matrix(dc6_file: &dc6_decoder::Dc6, idx: usize) -> Matrix {
    Matrix::from_dc6_encoded_frame(&dc6_file.directions[0].encoded_frames[idx])
}
