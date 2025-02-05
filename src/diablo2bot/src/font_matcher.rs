use std::collections::HashMap;

use crate::{
    enums::quality::Quality,
    matrix::Matrix,
    pattern_matcher2::{self, PatternMatcher2},
    quality_palette::QualityPalette,
    structs::{Character, Item, MatrixAndPoints, QualityCharacter, TrieOutput},
};

pub struct FontMatcher {
    pattern_matcher: PatternMatcher2<Character>,
}

impl FontMatcher {
    pub fn new(quality_palettes: &[QualityPalette], font_char_map: &HashMap<char, Matrix>) -> Self {
        let mut characters = Vec::new();

        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ':/"
            .chars()
            .for_each(|char| {
                let matrix = &font_char_map[&char];
                let width = matrix.get_non_zero_width() as u8;

                quality_palettes.iter().for_each(|palette| {
                    let matrix = matrix.palette_transform(&palette.palette);
                    let point_values = matrix.get_non_zero_point_values();

                    let matrix_and_points = MatrixAndPoints {
                        matrix,
                        point_values,
                    };

                    let character = Character {
                        char: QualityCharacter {
                            char,
                            width,
                            quality: palette.color,
                        },
                        matrix_and_points,
                    };

                    characters.push(character);
                })
            });

        let pattern_matcher = pattern_matcher2::PatternMatcher2::new(characters);

        Self { pattern_matcher }
    }

    pub fn match_image_items(&self, img: &Matrix) -> Vec<Item> {
        let font_chars_qualities = self.match_image_chars(img);
        let mut items = Vec::new();

        for (quality, chars) in font_chars_qualities {
            let mut name = String::new();
            let mut previous_width = chars[0].output.width;
            let mut previous_point = chars[0].point;
            let mut item_start_point = chars[0].point;

            for font_char in chars {
                if font_char.point.row != previous_point.row
                    || font_char.point.col as i16
                        - (previous_point.col as i16 + i16::from(previous_width))
                        >= 10
                        && !name.is_empty()
                {
                    items.push(Item {
                        name: name.clone(),
                        point: item_start_point,
                        quality,
                    });
                    name.clear();
                    item_start_point = font_char.point;
                }

                if font_char.point.col as i16
                    - (previous_point.col as i16 + i16::from(previous_width))
                    > 3
                    && !name.is_empty()
                {
                    name.push(' ');
                }

                name.push(font_char.output.char);

                previous_point = font_char.point;
                previous_width = font_char.output.width;
            }
            items.push(Item {
                name: name.clone(),
                point: item_start_point,
                quality,
            });
        }

        items.sort_by(|a, b| a.point.cmp(&b.point));

        items
    }

    fn match_image_chars(
        &self,
        img: &Matrix,
    ) -> HashMap<Quality, Vec<TrieOutput<QualityCharacter>>> {
        let mut font_chars_qualities = HashMap::new();

        let matches = self.pattern_matcher.look_up(img);

        matches.iter().for_each(|m| {
            font_chars_qualities
                .entry(m.output.quality)
                .or_insert(Vec::new())
                .push(*m);
        });

        for vector in &mut font_chars_qualities.values_mut() {
            vector.sort_by(|a, b| a.point.cmp(&b.point));
        }

        font_chars_qualities
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{
        enums::act::Act, file_io::FileIo, font_char_map::get_non_control_ascii_char_font_map,
        image::Image, mpq_archives::archives::Archives, structs::Item,
        test_utils::test_utils::read_json,
    };

    use super::FontMatcher;

    #[test]
    fn test_font_matcher() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let font_dc6_bytes = archives.extract_font_16_bytes().unwrap();
        let font_dc6_file = font_dc6_bytes.parse();
        let font_char_map = get_non_control_ascii_char_font_map(&font_dc6_file);

        let act = Act::Act5;
        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);
        let quality_palettes = pal_pl2_bytes
            .extract_font_quality_palette_bytes()
            .get_palettes();

        let font_matcher = FontMatcher::new(&quality_palettes, &font_char_map);

        let folder_path = file_io.root.join("test_data").join("font");
        let img = Image::load_image(&folder_path.join("items.png"));

        let matrix = img.to_matrix(&palette_transformer);

        let items_file_path = folder_path.join("items.json");

        let expected_items: Result<Vec<Item>, _> = read_json(&items_file_path);

        let items = font_matcher.match_image_items(&matrix);

        match expected_items {
            Ok(expected_items) => {
                assert_eq!(items, expected_items);
            }
            Err(err) => {
                if let std::io::ErrorKind::NotFound = err.kind() {
                    eprintln!(
                        "File not found: {} - Saving results from current test as file",
                        err
                    );
                    File::create(&items_file_path)
                        .unwrap()
                        .write_all(serde_json::to_string_pretty(&items).unwrap().as_bytes())
                        .unwrap();
                    assert!(false);
                } else {
                    panic!("Error reading JSON file: {}", err);
                }
            }
        }
    }
}
