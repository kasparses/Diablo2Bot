use std::collections::{HashMap, HashSet};

use crate::constants;
use crate::matrix::Matrix;
use crate::mpq_archives::archives::Archives;
use crate::point_u16::PointU16;
use crate::structs::MatrixAndPoints;
use crate::table::{Table, TableMetaData};
use crate::{pattern_matcher2::PatternMatcher2, structs::ConsumableItem};
use constants::belt_items::BELT_ITEMS;

pub struct ConsumableItemsTableMatcher {
    pattern_matcher: PatternMatcher2<ConsumableItem>,
}

impl ConsumableItemsTableMatcher {
    pub fn new(archives: &mut Archives, font_char_map: &HashMap<char, Matrix>) -> Self {
        let consumable_items = load_consumable_items(archives, font_char_map);

        Self {
            pattern_matcher: PatternMatcher2::new(consumable_items),
        }
    }

    pub fn match_from_matrix(&self, matrix: &Matrix, table_meta_data: TableMetaData) -> Table {
        let mut table = Table::new(table_meta_data);

        for (row, row_cells) in table.cells.iter_mut().enumerate() {
            for (col, cell) in row_cells.iter_mut().enumerate() {
                let img_point = table_meta_data.get_point(PointU16::new(row as u16, col as u16));

                let matches = self.pattern_matcher.look_up_point(matrix, img_point);

                if !matches.is_empty() {
                    *cell = Some(matches[0].output.clone());
                }
            }
        }

        table
    }
}

fn get_keybind_numbers_points(font_char_map: &HashMap<char, Matrix>) -> Vec<PointU16> {
    let mut points = HashSet::new();

    "1234".chars().for_each(|c| {
        font_char_map[&c]
            .get_non_zero_points()
            .iter()
            .for_each(|p| {
                points.insert(*p);
            })
    });

    points.into_iter().collect()
}

fn load_consumable_items(
    archives: &mut Archives,
    font_char_map: &HashMap<char, Matrix>,
) -> Vec<ConsumableItem> {
    let keybind_numbers_points = get_keybind_numbers_points(font_char_map);
    let mut consumable_items = Vec::new();

    for belt_item in BELT_ITEMS {
        let dc6_bytes = archives
            .extract_item_inventory_sprite(belt_item.inventory_sprite_file_name)
            .unwrap();
        let dc6 = dc6_bytes.parse();

        let mut matrix = Matrix::from_dc6_encoded_frame(&dc6.directions[0].encoded_frames[0]);

        keybind_numbers_points.iter().for_each(|point| {
            matrix.set_value(
                PointU16 {
                    row: point.row + 11,
                    col: point.col + 2,
                },
                0,
            );
        });

        let matrix_and_points = MatrixAndPoints {
            matrix: matrix.to_owned(),
            point_values: matrix.get_non_zero_point_values(),
        };
        let consumable_item = ConsumableItem {
            name: belt_item.name.to_string(),
            matrix_and_points,
        };
        consumable_items.push(consumable_item);
    }

    consumable_items
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    use crate::{
        enums::act::Act,
        file_io::FileIo,
        font_char_map::get_non_control_ascii_char_font_map,
        image::Image,
        mpq_archives::archives::Archives,
        table::{Table, TableMetaData},
        test_utils::test_utils::{get_directory, read_json},
    };

    #[test]
    fn test_table_matcher() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let font_dc6_bytes = archives.extract_font_16_bytes().unwrap();
        let font_dc6_file = font_dc6_bytes.parse();
        let font_char_map = get_non_control_ascii_char_font_map(&font_dc6_file);

        let act = Act::Act3;
        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

        let consumable_item_matcher =
            super::ConsumableItemsTableMatcher::new(&mut archives, &font_char_map);

        let test_data_path = file_io.root.join("test_data");
        let dir = get_directory(&test_data_path.as_os_str().to_str().unwrap(), "table");

        for test_folder in dir.subdirectories.iter() {
            let table_meta_data: TableMetaData = read_json(Path::new(&format!(
                "{}/table_meta_data.json",
                test_folder.path
            )))
            .unwrap();

            let img = Image::load_image(&Path::new(&format!("{}/items.png", test_folder.path)));
            let matrix = img.to_matrix(&palette_transformer);
            let expected_table: Result<Table, _> =
                read_json(Path::new(&format!("{}/items.json", test_folder.path)));

            let table = consumable_item_matcher.match_from_matrix(&matrix, table_meta_data);

            match expected_table {
                Ok(expected_table) => {
                    assert_eq!(table, expected_table);
                }
                Err(err) => {
                    if let std::io::ErrorKind::NotFound = err.kind() {
                        eprintln!(
                            "File not found: {} - Saving results from current test as file",
                            err
                        );
                        File::create(&format!("{}/items.json", test_folder.path))
                            .unwrap()
                            .write_all(serde_json::to_string_pretty(&table).unwrap().as_bytes())
                            .unwrap();
                        assert!(false);
                    } else {
                        panic!("Error reading JSON file: {}", err);
                    }
                }
            }
        }
    }
}
