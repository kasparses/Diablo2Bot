use crate::bot_settings::MovementSettings;
use crate::constants::directions::{DIRECTIONS, DIRECTIONS2};
use crate::constants::map_tile_masks::MapSpriteCell;
use crate::constants::misc::MAP_SPRITES_MATRIX_SIZE;
use crate::enums::errors::{CouldNotConnectMapsError, CouldNotGetPathError};
use crate::image::Image;
use crate::logger::Logger;
use crate::pal_pl2::PixelPalette;
use crate::point_u16::DirectionEnum;
use crate::{
    map_matcher::MapMatcher, matrix::Matrix, point_i32::PointI32, point_u16::PointU16,
    tile_mask_getter::TileMaskGetter,
};
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use std::mem;

fn swap_vectors<T>(vec1: &mut Vec<T>, vec2: &mut Vec<T>) {
    mem::swap(vec1, vec2);
}

pub struct Direction {
    pub point: PointI32,
    pub is_vertical: bool,
}

#[derive(Clone, Copy)]
struct MapTileInfo {
    walked_count: u32,
    walked_count_path: u32,
    walkable: MapSpriteCell,
    steps_to_base: Option<u32>,
}

#[derive(Debug)]
struct PointMapTileInfo {
    point: PointU16,
    new_area_ratio: f64,
}

struct MapTileMatrix {
    dims: PointU16,
    data: Vec<MapTileInfo>,
    wide_start_size: u32,
}

impl MapTileMatrix {
    fn new(dims: PointU16, wide_start_size: u32) -> Self {
        let data = vec![
            MapTileInfo {
                walked_count: 0,
                walked_count_path: 0,
                walkable: MapSpriteCell::Empty,
                steps_to_base: None
            };
            dims.len() as usize
        ];

        Self {
            dims,
            data,
            wide_start_size,
        }
    }

    pub fn pad(&mut self, pad_stats: PadStats) {
        if pad_stats.top == 0
            && pad_stats.bottom == 0
            && pad_stats.left == 0
            && pad_stats.right == 0
        {
            return;
        }

        let new_dims = PointU16 {
            row: self.dims.row + pad_stats.top as u16 + pad_stats.bottom as u16,
            col: self.dims.col + pad_stats.left as u16 + pad_stats.right as u16,
        };

        let mut new_data = vec![
            MapTileInfo {
                walked_count: 0,
                walked_count_path: 0,
                walkable: MapSpriteCell::Empty,
                steps_to_base: None
            };
            new_dims.len() as usize
        ];

        for row in 0..u32::from(self.dims.row) {
            for col in 0..u32::from(self.dims.col) {
                let old_index = (row * u32::from(self.dims.col) + col) as usize;

                let new_row = row + pad_stats.top;
                let new_col = col + pad_stats.left;

                let new_index = (new_row * u32::from(new_dims.col) + new_col) as usize;

                new_data[new_index] = self.data[old_index];
            }
        }

        self.dims = new_dims;
        self.data = new_data;
    }

    fn is_edge_point(&self, point: PointU16) -> bool {
        point.row == 0
            || point.row == self.dims.row - 1
            || point.col == 0
            || point.col == self.dims.col - 1
    }

    fn mark_unwalkable_tiles(
        &mut self,
        tile_mask_getter: &TileMaskGetter,
        map_sprites: &HashMap<u32, Vec<PointU16>>,
        offset: PointU16,
    ) {
        for (sprite_id, points) in map_sprites {
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
                        let idx = self.get_index(offset + *point + p);
                        if self.data[idx].walkable != MapSpriteCell::Opening {
                            self.data[idx].walkable = *col;
                        }
                    }
                }
            }
        }
    }

    fn get_index(&self, point: PointU16) -> usize {
        (u32::from(point.row) * u32::from(self.dims.col) + u32::from(point.col)) as usize
    }

    fn fill_maze(&mut self, base_point: PointU16, use_wide_start: bool) {
        self.reset_steps_to_base_and_walked_count_path();

        let idx = self.get_index(base_point);
        self.data[idx].steps_to_base = Some(0);

        let mut current_positions: Vec<PointU16> = vec![base_point];
        let mut next_positions: Vec<PointU16> = Vec::new();

        let wide_start_size = self.wide_start_size as u16;

        if use_wide_start {
            for row in base_point.row - wide_start_size..base_point.row + wide_start_size {
                for col in base_point.col - wide_start_size..base_point.col + wide_start_size {
                    let point = PointU16::new(row, col);
                    let idx = self.get_index(point);
                    self.data[idx].steps_to_base = Some(0);
                    current_positions.push(point);
                }
            }
        }

        let limit = 72;

        for step_num in 0..limit {
            for position in &current_positions {
                if self.is_edge_point(*position) {
                    continue;
                }

                let current_position_idx = self.get_index(*position);
                let current_position_walked_count = self.data[current_position_idx].walked_count;
                let current_position_walked_count_path =
                    self.data[current_position_idx].walked_count_path;

                for direction in DIRECTIONS {
                    let next_position = PointU16::new(
                        (i32::from(position.row) + direction.row) as u16,
                        (i32::from(position.col) + direction.col) as u16,
                    );

                    let next_position_idx = self.get_index(next_position);

                    if self.data[next_position_idx].walkable.is_walkable()
                        && self.data[next_position_idx].steps_to_base.is_none()
                    {
                        self.data[next_position_idx].steps_to_base = Some(step_num);

                        next_positions.push(next_position);
                        self.data[next_position_idx].walked_count_path +=
                            current_position_walked_count_path;
                        self.data[next_position_idx].walked_count_path +=
                            current_position_walked_count;
                    }
                }
            }

            swap_vectors(&mut current_positions, &mut next_positions);
            next_positions.clear();
        }
    }

    fn get_random_destination_points(&self, max_num_points: u32) -> Vec<PointMapTileInfo> {
        let mut point_map_tile_info = Vec::new();

        let mut rng = rand::thread_rng();

        let limit = 65536;
        let mut c = 0;

        while point_map_tile_info.len() < max_num_points as usize && c < limit {
            c += 1;

            let row: u16 = rng.gen_range(0..self.dims.row);
            let col: u16 = rng.gen_range(0..self.dims.col);
            let point = PointU16::new(row, col);

            let idx = self.get_index(point);

            if let Some(steps) = self.data[idx].steps_to_base {
                if steps < 10 {
                    continue;
                }

                point_map_tile_info.push(PointMapTileInfo {
                    point,
                    new_area_ratio: (f64::from(self.data[idx].walked_count_path)).sqrt()
                        / f64::from(steps),
                });
            }
        }

        point_map_tile_info
    }

    fn get_path(&self, end_point: PointU16) -> Option<Vec<PointU16>> {
        let idx = self.get_index(end_point);

        self.data[idx].steps_to_base?;

        let mut current_position_num_steps_to_start_position =
            self.data[idx].steps_to_base.unwrap();

        let mut path = vec![end_point];

        let limit = 65536;
        let mut c = 0;

        let mut current_position = end_point;

        let mut is_last_step_vertical = true;

        while current_position_num_steps_to_start_position > 0 && c < limit {
            c += 1;

            let mut next_step = None;
            let mut num_steps = 0;

            for direction in DIRECTIONS2 {
                let adjacent_position_row = i32::from(current_position.row) + direction.point.row;
                let adjacent_position_col = i32::from(current_position.col) + direction.point.col;

                if adjacent_position_row < 0
                    || adjacent_position_row >= i32::from(self.dims.row)
                    || adjacent_position_col < 0
                    || adjacent_position_col >= i32::from(self.dims.col)
                {
                    continue;
                }

                let adjacent_position =
                    PointU16::new(adjacent_position_row as u16, adjacent_position_col as u16);

                let adjacent_position_idx = self.get_index(adjacent_position);

                if let Some(steps) = self.data[adjacent_position_idx].steps_to_base {
                    if steps < current_position_num_steps_to_start_position {
                        next_step = Some(adjacent_position);
                        num_steps = steps;

                        if direction.is_vertical != is_last_step_vertical {
                            is_last_step_vertical = !is_last_step_vertical;
                            break;
                        }
                    }
                }
            }

            current_position = next_step.unwrap();
            current_position_num_steps_to_start_position = num_steps;
            path.push(current_position);
        }

        path.reverse();

        Some(path)
    }

    fn reset_steps_to_base_and_walked_count_path(&mut self) {
        for map_tile_data in &mut self.data {
            map_tile_data.steps_to_base = None;
            map_tile_data.walked_count_path = 0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TotalMovement {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

#[derive(Debug, Clone, Copy)]
struct PadStats {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

fn get_pad_stats(total_movement: TotalMovement, current_point: PointI32) -> PadStats {
    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;

    if current_point.row > total_movement.down as i32 {
        bottom = current_point.row - total_movement.down as i32;
    }

    if current_point.col > total_movement.right as i32 {
        right = current_point.col - total_movement.right as i32;
    }

    if current_point.row < -(total_movement.up as i32) {
        top = i32::abs(current_point.row) - total_movement.up as i32;
    }

    if current_point.col < -(total_movement.left as i32) {
        left = i32::abs(current_point.col) - total_movement.left as i32;
    }

    PadStats {
        top: top as u32,
        bottom: bottom as u32,
        left: left as u32,
        right: right as u32,
    }
}

fn get_new_distance_to_starting_point(
    current_distance_to_starting_point: PointI32,
    diff: PointI32,
) -> PointI32 {
    let row = current_distance_to_starting_point.row + diff.row;
    let col = current_distance_to_starting_point.col + diff.col;

    PointI32::new(row, col)
}

fn get_current_window_offset_point(
    total_movement: TotalMovement,
    current_point: PointI32,
) -> PointU16 {
    let row = (total_movement.up as i32 + current_point.row) as u16;
    let col = (total_movement.left as i32 + current_point.col) as u16;

    PointU16::new(row, col)
}

fn adjust_total_movement(
    total_movement: TotalMovement,
    current_distance_to_starting_point: PointI32,
) -> TotalMovement {
    TotalMovement {
        up: i32::abs(i32::min(
            -(total_movement.up as i32),
            current_distance_to_starting_point.row,
        )) as u32,
        down: i32::max(
            total_movement.down as i32,
            current_distance_to_starting_point.row,
        ) as u32,
        left: i32::abs(i32::min(
            -(total_movement.left as i32),
            current_distance_to_starting_point.col,
        )) as u32,
        right: i32::max(
            total_movement.right as i32,
            current_distance_to_starting_point.col,
        ) as u32,
    }
}

fn get_previous_mid_point(current_mid_point: PointU16, pad_stats: PadStats) -> PointU16 {
    let row = current_mid_point.row + pad_stats.top as u16;
    let col = current_mid_point.col + pad_stats.left as u16;

    PointU16::new(row, col)
}

pub struct ZoneTraveller {
    movement_settings: MovementSettings,
    tile_mask_getter: TileMaskGetter,
    total_movement: TotalMovement,
    current_distance_to_starting_point: PointI32,
    current_mid_point: PointU16,
    map_sprites: HashMap<u32, Vec<PointU16>>,
    map_tile_matrix: MapTileMatrix,
    use_wide_start_for_next_path: bool,
    last_attempted_direction: DirectionEnum,
    was_last_move_succesful: bool,
}

impl ZoneTraveller {
    pub fn new(
        movement_settings: MovementSettings,
        map_sprite_matcher: &MapMatcher,
        matrix: Matrix,
    ) -> Self {
        let tile_mask_getter = TileMaskGetter::new();
        let total_movement = TotalMovement {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        };

        let current_distance_to_starting_point = PointI32::new(0, 0);
        let current_mid_point = PointU16::new(75, 50);

        let mut map_tile_matrix =
            MapTileMatrix::new(MAP_SPRITES_MATRIX_SIZE, movement_settings.wide_start_size);

        let map_sprites = map_sprite_matcher.match_map_sprites(&matrix);
        map_tile_matrix.mark_unwalkable_tiles(&tile_mask_getter, &map_sprites, PointU16::new(0, 0));

        Self {
            tile_mask_getter,
            total_movement,
            current_distance_to_starting_point,
            current_mid_point,
            map_sprites,
            map_tile_matrix,
            use_wide_start_for_next_path: false,
            last_attempted_direction: DirectionEnum::Same,
            was_last_move_succesful: true,
            movement_settings,
        }
    }

    pub fn get_path(
        &mut self,
        pixel_palette: &PixelPalette,
        logger: &mut Logger,
    ) -> Result<Vec<PointU16>, CouldNotGetPathError> {
        self.map_tile_matrix
            .fill_maze(self.current_mid_point, self.use_wide_start_for_next_path);

        if self.was_last_move_succesful {
            let random_destination_points = self.map_tile_matrix.get_random_destination_points(
                self.movement_settings
                    .num_random_destination_points_to_choose_from,
            );

            // TODO Udregn for alle points hvor tæt de er på en væg og inkluder det i min rangering.
            // Det er best ikke at komme for tæt på en væg.
            let best_destination_point = match random_destination_points
                .iter()
                .min_by(|a, b| a.new_area_ratio.total_cmp(&b.new_area_ratio))
            {
                Some(point) => point,
                None => return Err(CouldNotGetPathError),
            };

            let path = self.map_tile_matrix.get_path(best_destination_point.point);
            match path {
                Some(path) => {
                    let path = if path.len() > 10 {
                        path
                    } else {
                        let direction = DirectionEnum::get_random_diagonal_direction();

                        self.current_mid_point.move_in_direction(direction, 64)
                    };

                    self.log(&path, pixel_palette, true, logger);
                    let path_to_mark = &path[..(self
                        .movement_settings
                        .max_num_tiles_from_path_to_mark_as_walked
                        as usize)
                        .min(path.len())];
                    self.mark_path_as_walked(path_to_mark);

                    self.last_attempted_direction = self
                        .current_mid_point
                        .direction_to(&best_destination_point.point);

                    Ok(path)
                }
                None => Err(CouldNotGetPathError),
            }
        } else {
            println!("Moving in opposite direction!");
            let opposite_direction = self.last_attempted_direction.opposite();
            self.last_attempted_direction = opposite_direction;

            let path = self
                .current_mid_point
                .move_in_direction(opposite_direction, 64);
            self.log(&path, pixel_palette, true, logger);
            let path_to_mark = &path[..(self
                .movement_settings
                .max_num_tiles_from_path_to_mark_as_walked
                as usize)
                .min(path.len())];
            self.mark_path_as_walked(path_to_mark);

            Ok(path)
        }
    }

    fn mark_path_as_walked(&mut self, path: &[PointU16]) {
        let mut points = HashSet::new();
        for point in path {
            Self::add_surrounding_points(&mut points, *point, 8, self.map_tile_matrix.dims);
        }

        for point in points {
            let idx = self.map_tile_matrix.get_index(point);
            self.map_tile_matrix.data[idx].walked_count += 1;
        }
    }

    fn add_surrounding_points(
        points: &mut HashSet<PointU16>,
        point: PointU16,
        radius: u8,
        dims: PointU16,
    ) {
        let start_row = (i32::from(point.row) - i32::from(radius)).max(0) as u16;
        let end_row = (i32::from(point.row) + i32::from(radius)).min(i32::from(dims.row)) as u16;

        let start_col = (i32::from(point.col) - i32::from(radius)).max(0) as u16;
        let end_col = (i32::from(point.col) + i32::from(radius)).min(i32::from(dims.col)) as u16;

        for row in start_row..end_row {
            for col in start_col..end_col {
                let new_point = PointU16::new(row, col);
                points.insert(new_point);
            }
        }
    }

    pub fn log(
        &mut self,
        path: &[PointU16],
        pixel_palette: &PixelPalette,
        is_planned_path: bool,
        logger: &mut Logger,
    ) {
        if !logger.save_logs() {
            return;
        }

        let matrix = self.get_walkable_as_matrix();
        let mut img = matrix.to_image(pixel_palette);
        self.draw_walked_count(&mut img);
        if is_planned_path {
            draw_path(path, &mut img);
            logger.log_image("zone_traveller", "planned_path", &img);
        } else {
            draw_path(path, &mut img);
            logger.log_image("zone_traveller", "last_center_to_current_center", &img);
        }
    }

    fn count_matches(map_sprites: &HashMap<u32, Vec<PointU16>>) -> u32 {
        let mut total = 0;

        for (_, sprites) in map_sprites.iter() {
            total += sprites.len() as u32;
        }

        total
    }

    pub fn update_map(
        &mut self,
        map_sprite_matcher: &MapMatcher,
        matrix: &Matrix,
        pixel_palette: &PixelPalette,
        logger: &mut Logger,
    ) -> Result<(), CouldNotConnectMapsError> {
        let map_sprites = map_sprite_matcher.match_map_sprites(matrix);

        let num_matches_previous = Self::count_matches(&self.map_sprites);
        let num_matches_current = Self::count_matches(&map_sprites);
        let ratio = num_matches_current as f64 / num_matches_previous as f64;

        if ratio < 0.5 {
            // If we suddendly match much fewer map sprites it could indicate that we have not extracted the map sprites correctly.
            return Err(CouldNotConnectMapsError);
        }

        let diff = Self::get_diff(&self.map_sprites, &map_sprites);

        self.current_distance_to_starting_point =
            get_new_distance_to_starting_point(self.current_distance_to_starting_point, diff);

        let pad_stats = get_pad_stats(self.total_movement, self.current_distance_to_starting_point);
        self.map_tile_matrix.pad(pad_stats);

        self.total_movement =
            adjust_total_movement(self.total_movement, self.current_distance_to_starting_point);

        let current_window_offset_point = get_current_window_offset_point(
            self.total_movement,
            self.current_distance_to_starting_point,
        );

        let previous_window_mid_point = get_previous_mid_point(self.current_mid_point, pad_stats);

        let current_window_mid_point = PointU16::new(
            current_window_offset_point.row + 75,
            current_window_offset_point.col + 50,
        );
        self.current_mid_point = current_window_mid_point;

        self.map_tile_matrix.mark_unwalkable_tiles(
            &self.tile_mask_getter,
            &map_sprites,
            current_window_offset_point,
        );

        self.map_tile_matrix.fill_maze(self.current_mid_point, true);

        let path_from_current_mid_to_previous_mid =
            self.map_tile_matrix.get_path(previous_window_mid_point);
        match path_from_current_mid_to_previous_mid {
            Some(path) => {
                self.log(&path, pixel_palette, false, logger);
                assert!(!path.is_empty());

                self.was_last_move_succesful = if diff.row.abs() + diff.col.abs() > 10 {
                    self.use_wide_start_for_next_path = false;
                    true
                } else {
                    self.use_wide_start_for_next_path = true;
                    false
                };

                self.current_mid_point = path[0];
                self.map_sprites = map_sprites;

                Ok(())
            }
            None => Err(CouldNotConnectMapsError),
        }
    }

    fn get_diff(
        map_sprites_1: &HashMap<u32, Vec<PointU16>>,
        map_sprites_2: &HashMap<u32, Vec<PointU16>>,
    ) -> PointI32 {
        let mut offsets: HashMap<PointI32, u32> = HashMap::new();

        for (sprite_id, points_1) in map_sprites_1 {
            if let Some(points_2) = map_sprites_2.get(sprite_id) {
                for point_1 in points_1 {
                    for point_2 in points_2 {
                        let offset = PointI32 {
                            row: (i32::from(point_1.row)) - (i32::from(point_2.row)),
                            col: (i32::from(point_1.col)) - (i32::from(point_2.col)),
                        };

                        *offsets.entry(offset).or_insert(0) += 1;
                    }
                }
            }
        }

        offsets
            .iter()
            .max_by_key(|&(_, count)| count)
            .map_or(PointI32::new(0, 0), |(&key, _)| key)
    }

    fn get_walkable_as_matrix(&self) -> Matrix {
        let map_tile_matrix = &self.map_tile_matrix;
        let mut matrix = Matrix::new_empty(map_tile_matrix.dims);

        for row in 0..map_tile_matrix.dims.row {
            for col in 0..map_tile_matrix.dims.col {
                let point = PointU16::new(row, col);
                let idx = map_tile_matrix.get_index(point);
                if !map_tile_matrix.data[idx].walkable.is_walkable() {
                    matrix.set_value(point, 255);
                }
            }
        }

        matrix
    }

    fn draw_walked_count(&self, img: &mut Image) {
        for row in 0..self.map_tile_matrix.dims.row {
            for col in 0..self.map_tile_matrix.dims.col {
                let point = PointU16::new(row, col);
                let idx = self.map_tile_matrix.get_index(point);
                if self.map_tile_matrix.data[idx].walkable.is_walkable() {
                    let walked_count = self.map_tile_matrix.data[idx].walked_count;

                    let color = (walked_count * 30).min(255) as u8;

                    let idx = (point.row as u32 * img.dims.col as u32 + point.col as u32) as usize;
                    img.pixels[idx].blue = color;
                    img.pixels[idx].green = color;
                }
            }
        }
    }
}

fn draw_path(path: &[PointU16], img: &mut Image) {
    for point in path {
        let idx = (point.row as u32 * img.dims.col as u32 + point.col as u32) as usize;

        img.pixels[idx].red = 255;
    }

    if let Some(point) = path.first() {
        let idx = (point.row as u32 * img.dims.col as u32 + point.col as u32) as usize;

        img.pixels[idx].green = 255;
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    use crate::{
        enums::act::Act, file_io::FileIo, image::Image, logger::Logger, map_matcher::MapMatcher,
        mpq_archives::archives::Archives, test_utils::test_utils::get_directory,
    };

    use super::{draw_path, ZoneTraveller};

    #[test]
    fn test_zone_traveller() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let bot_settings = file_io.load_bot_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(Act::Act1.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

        let map_sprite_matcher = MapMatcher::new("1 Wilderness", &mut archives);

        let folder_path = "/home/kasper/code/diablo2bot/test_data/zone_traveller/area/1 Wilderness";

        let dir = get_directory(&folder_path, "0");
        let matrix =
            Image::load_image(Path::new(&dir.files[0].path)).to_matrix(&palette_transformer);
        let mut zone_traveller =
            ZoneTraveller::new(bot_settings.movement_settings, &map_sprite_matcher, matrix);

        let mut logger = Logger::new(&file_io, &bot_settings);

        for file in dir.files[1..].iter() {
            let path = zone_traveller
                .get_path(&pixel_palette, &mut logger)
                .unwrap();
            let matrix = zone_traveller.get_walkable_as_matrix();
            let mut img = matrix.to_image(&pixel_palette);
            draw_path(&path, &mut img);
            zone_traveller.draw_walked_count(&mut img);
            let idx = zone_traveller
                .map_tile_matrix
                .get_index(zone_traveller.current_mid_point);
            img.pixels[idx].green = 100;
            img.save_image(&Path::new(&format!(
                "{}/test_output/{}",
                folder_path, file.name
            )))
            .unwrap();

            let matrix = Image::load_image(&Path::new(&file.path)).to_matrix(&palette_transformer);
            zone_traveller
                .update_map(&map_sprite_matcher, &matrix, &pixel_palette, &mut logger)
                .unwrap();
        }
    }
}
