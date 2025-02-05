use std::collections::HashMap;

use crate::{
    box_u16::BoxU16,
    fast_hash_set::FastHashSet,
    matrix::Matrix,
    mpq_archives::archives::Archives,
    pattern_matcher::PatternMatcher,
    point_u16::PointU16,
    structs::{MatrixAndPoints2, PointIndex},
};

// https://d2mods.info/forum/kb/viewarticle?a=419

pub struct MapMatcher {
    pattern_matcher: PatternMatcher,
    step_size: PointU16,
}

impl MapMatcher {
    pub fn new(area_name: &str, archives: &mut Archives) -> Self {
        const STEP_SIZE: PointU16 = PointU16 { row: 4, col: 8 };

        let excel_automap_raw_text = archives.extract_excel_automap_raw_text().unwrap();
        let excel_automap = excel_automap_raw_text.parse();
        let area_map_sprite_ids = excel_automap.get_map_sprite_ids_for_area(area_name);

        let map_sprites_dc6_bytes = archives.extract_map_sprites().unwrap();
        let map_sprites_dc6_file = map_sprites_dc6_bytes.parse();

        let mut sprites = FastHashSet::new();

        let matrices: Vec<MatrixAndPoints2> = area_map_sprite_ids
            .iter()
            .filter_map(|sprite_id| {
                let matrix = Matrix::from_dc6_encoded_frame(
                    &map_sprites_dc6_file.directions[0].encoded_frames[*sprite_id as usize],
                );

                let is_new_sprite = sprites.insert(&matrix.data);

                if is_new_sprite {
                    let point_values = matrix.get_non_zero_point_values();

                    Some(MatrixAndPoints2 {
                        sprite_id: *sprite_id,
                        matrix,
                        point_values,
                    })
                } else {
                    None
                }
            })
            .collect();

        let pattern_matcher = PatternMatcher::new(matrices);

        Self {
            pattern_matcher,
            step_size: STEP_SIZE,
        }
    }

    pub fn match_map_sprites(&self, matrix: &Matrix) -> HashMap<u32, Vec<PointU16>> {
        let mut matches = self.match_map_sprites_(matrix);
        matches.sort_by(|a, b| a.point.cmp(&b.point));

        let mut sprite_id_to_points = HashMap::new();

        matches
            .iter()
            .filter(|point_index| !Self::ignore_point(point_index.point))
            .for_each(|point_index| {
                let point = PointU16 {
                    row: point_index.point.row / 4,
                    col: point_index.point.col / 8,
                };
                sprite_id_to_points
                    .entry(point_index.sprite_id)
                    .or_insert_with(Vec::new)
                    .push(point);
            });

        sprite_id_to_points
    }

    fn ignore_point(point: PointU16) -> bool {
        point.row < 16 || point.row > 520 || point.col < 16 || point.col > 776
    }

    fn get_middle_area(&self) -> BoxU16 {
        let area = BoxU16 {
            offset: PointU16 { row: 200, col: 304 },
            dimensions: PointU16 { row: 200, col: 200 },
        };

        assert!(area.offset.col % self.step_size.col == 0);
        assert!(area.dimensions.col % self.step_size.col == 0);
        assert!(area.offset.row % self.step_size.row == 0);
        assert!(area.dimensions.row % self.step_size.row == 0);

        area
    }

    fn get_point_offset(&self, img: &Matrix) -> PointU16 {
        let middle_area = self.get_middle_area();

        let step_box: BoxU16 = self.step_size.into();

        step_box
            .iter_box_points()
            .map(|point| {
                let points: Vec<PointU16> = middle_area
                    .offset(point)
                    .iter_box_points_with_step(self.step_size)
                    .collect();
                let mut img_mask = Matrix::new_empty(img.dims);

                let match_count = self.match_map_sprites__(img, &mut img_mask, &points).len();
                (point, match_count)
            })
            .max_by_key(|&(_, count)| count)
            .map(|(point, _)| point)
            .unwrap_or_else(|| PointU16 { row: 0, col: 0 })
    }

    fn get_initial_points(&self, img: &Matrix, point_offset: PointU16) -> Vec<PointU16> {
        let dims = self.pattern_matcher.matrix_dimensions;

        let area = BoxU16 {
            offset: point_offset,
            dimensions: img.dims - dims,
        };

        area.iter_box_points_with_step(self.step_size).collect()
    }

    fn match_map_sprites_(&self, img: &Matrix) -> Vec<PointIndex> {
        let mut img_mask = Matrix::new_empty(img.dims);
        let point_offset = self.get_point_offset(img);
        let points = self.get_initial_points(img, point_offset);

        Self::add_edges_to_img_mask(&mut img_mask);

        self.pattern_matcher
            .find_map_sprites(img, &mut img_mask, &points)
    }

    fn match_map_sprites__(
        &self,
        img: &Matrix,
        img_mask: &mut Matrix,
        points: &[PointU16],
    ) -> Vec<PointIndex> {
        self.pattern_matcher.find_map_sprites(img, img_mask, points)
    }

    fn add_edges_to_img_mask(img_mask: &mut Matrix) {
        let size = BoxU16 {
            offset: PointU16 { row: 8, col: 16 },
            dimensions: PointU16 { row: 0, col: 8 },
        };

        for row in 0..size.offset.row {
            for col in 0..img_mask.dims.col {
                img_mask.set_value(PointU16 { row, col }, 1);
            }
        }

        for row in size.offset.row..img_mask.dims.row {
            for col in 0..size.offset.col {
                img_mask.set_value(PointU16 { row, col }, 1);
            }

            for col in (img_mask.dims.col - size.dimensions.col)..img_mask.dims.col {
                img_mask.set_value(PointU16 { row, col }, 1);
            }
        }

        for row in 553..img_mask.dims.row {
            for col in 0..img_mask.dims.col {
                img_mask.set_value(PointU16 { row, col }, 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, fs};

    use crate::{
        constants::{map_tile_masks::MapSpriteCell, misc::MAP_SPRITES_MATRIX_SIZE},
        enums::act::Act,
        file_io::FileIo,
        image::Image,
        matrix::Matrix,
        mpq_archives::archives::Archives,
        point_u16::PointU16,
        tile_mask_getter::TileMaskGetter,
    };

    use super::MapMatcher;

    fn draw_map_sprites(
        map_sprites: HashMap<u32, Vec<PointU16>>,
        tile_mask_getter: &TileMaskGetter,
    ) -> Matrix {
        let mut matrix = Matrix::new_empty(MAP_SPRITES_MATRIX_SIZE);

        for (sprite_id, points) in map_sprites.iter() {
            let mask = tile_mask_getter.get_tile_mask(*sprite_id);

            for (i, row) in mask.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    if *col == MapSpriteCell::Empty {
                        continue;
                    }

                    let p = PointU16 {
                        row: i as u16,
                        col: j as u16,
                    };

                    for point in points {
                        matrix.set_value(*point + p, 255);
                    }
                }
            }
        }

        matrix
    }

    #[test]
    fn test_map_matcher() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();

        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let folder_path = file_io.root.join("test_data").join("map").join("palette");

        let tile_mask_getter = TileMaskGetter::new();

        fs::read_dir(folder_path).unwrap().for_each(|folder| {
            let folder = folder.unwrap();
            let mut folder_path = folder.path();
            let folder_path2 = folder_path.clone();
            let act_folder_name = folder_path2.file_name().unwrap().to_str().unwrap();

            let pal_pl2_bytes = archives
                .extract_pal_pl2_bytes(Act::from_str(act_folder_name).into())
                .unwrap();
            let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
            let pixel_palette = act_palette_bytes.get_pixel_palette();
            let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

            folder_path.push("area");

            fs::read_dir(folder_path).unwrap().for_each(|folder| {
                let folder = folder.unwrap();
                let folder_path = folder.path();
                let folder_path2 = folder_path.clone();
                let area_folder_name = folder_path2.file_name().unwrap().to_str().unwrap();

                let map_matcher = MapMatcher::new(area_folder_name, &mut archives);

                fs::read_dir(folder_path).unwrap().for_each(|folder| {
                    let folder = folder.unwrap();
                    let folder_path = folder.path();

                    let full_image = Image::load_image(&folder_path.join("full_image.png"));
                    let matrix = full_image.to_matrix(&palette_transformer);

                    let map_sprites = map_matcher.match_map_sprites(&matrix);

                    let map_sprites_matrix = draw_map_sprites(map_sprites, &tile_mask_getter);

                    let actual_isolated_image = map_sprites_matrix.to_image(&pixel_palette);

                    actual_isolated_image
                        .save_image(&folder_path.join("isolated_map_black_and_white.png"))
                        .unwrap();
                });
            });
        });
    }
}
