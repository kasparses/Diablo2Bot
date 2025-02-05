use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Deref,
    time::Instant,
};

use arrayvec::ArrayVec;
use dcc_decoder::Dcc;
use nohash_hasher::BuildNoHashHasher;
use serde::{Deserialize, Serialize};

use crate::{
    bot_settings::BotSettings,
    enums::{
        act::Act, errors::BotError, game_difficulty::GameDifficulty,
        monster_matcher_type::MonsterMatcherType,
    },
    fast_hash_set::FastHashSet,
    file_io::FileIo,
    level_name::LevelName,
    matrix::Matrix,
    mpq_archives::archives::Archives,
    palette::Palette,
    point_u16::PointU16,
    string_tables::ZoneNameConverter,
    zone_monsters::{get_monster, get_monsters_in_level, DccFile, Monster},
};

const WINDOW_SIZE: usize = 16;
const MATRIX_PALETTE_SIZE: usize = 6;

struct Palettes {
    data: Vec<Palette>,
}

struct Matrices {
    data: Vec<[u8; WINDOW_SIZE]>,
    matrix_dimensions: PointU16,
    palettes: Palettes,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct MatrixPalette {
    matrix_id: u32,
    palette_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeMatch {
    matrix_palette: MatrixPalette,
    pub window_offset_point: PointU16,
}

pub struct Tree {
    data: Vec<u8>,
    matrices: Matrices,
    pub level: String,
}

struct SpriteWindow {
    data: [u8; WINDOW_SIZE],
    num_unique_values: u32,
}

struct SpriteWindows {
    windows: Vec<SpriteWindow>,
}

pub struct TreeCacheFiles {
    pub tree_data: Vec<u8>,
    pub matrices_data: Vec<u8>,
    pub matrices_palettes: Vec<u8>,
}

pub struct TreeCacheFilesBorrowed<'a> {
    pub tree_data: &'a Vec<u8>,
    pub matrices_data: &'a Vec<u8>,
    pub matrices_palettes: &'a Vec<u8>,
}

impl MatrixPalette {
    fn get_bytes(self) -> [u8; MATRIX_PALETTE_SIZE] {
        let m = self.matrix_id.to_be_bytes();
        let p = self.palette_id.to_be_bytes();

        [m[0], m[1], m[2], m[3], p[0], p[1]]
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let matrix_id_bytes: [u8; 4] = bytes[0..4].try_into().expect("slice with incorrect length");
        let matrix_id: u32 = u32::from_be_bytes(matrix_id_bytes);

        let palette_id_bytes: [u8; 2] =
            bytes[4..6].try_into().expect("slice with incorrect length");
        let palette_id: u16 = u16::from_be_bytes(palette_id_bytes);

        Self {
            matrix_id,
            palette_id,
        }
    }
}

fn split_window_offset_points(
    img: &Matrix,
    value: u8,
    matrix_point: PointU16,
    mask: &[bool; 256],
    splits: &mut [[Vec<PointU16>; 256]],
    level: u8,
) {
    for i in 0..splits[level as usize - 1][value as usize].len() {
        let window_offset_point = splits[level as usize - 1][value as usize][i];
        let value = img.get_value(window_offset_point + matrix_point);

        if mask[value as usize] {
            splits[level as usize][value as usize].push(window_offset_point);
        }
    }
}

fn split_matrix_palettes_with_array_vec(
    matrices: &Matrices,
    matrix_palettes: &[MatrixPalette],
    point: u32,
) -> HashMap<u8, ArrayVec<MatrixPalette, 8>, BuildNoHashHasher<u8>> {
    let mut split: HashMap<u8, ArrayVec<MatrixPalette, 8>, BuildNoHashHasher<u8>> =
        HashMap::default();

    for matrix_palette in matrix_palettes {
        let value = matrices.get_value(*matrix_palette, point);

        split.entry(value).or_default().push(*matrix_palette);
    }

    split
}

fn split_matrix_palettes_with_vec(
    matrices: &Matrices,
    matrix_palettes: &[MatrixPalette],
    point: u32,
) -> HashMap<u8, Vec<MatrixPalette>, BuildNoHashHasher<u8>> {
    let mut split: HashMap<u8, Vec<MatrixPalette>, BuildNoHashHasher<u8>> = HashMap::default();

    for matrix_palette in matrix_palettes {
        let value = matrices.get_value(*matrix_palette, point);

        split.entry(value).or_default().push(*matrix_palette);
    }

    split
}

impl Tree {
    fn new(matrices: Matrices, matrix_palettes: &[MatrixPalette], level: String) -> Self {
        assert!(!matrix_palettes.is_empty());

        let mut data = Vec::new();
        Self::create_tree(&matrices, &mut data, matrix_palettes, 0);

        data.shrink_to_fit();

        Self {
            data,
            matrices,
            level,
        }
    }

    fn save_to_cache(&self, file_io: &FileIo, name: &str) -> io::Result<()> {
        let tree_cache_files = TreeCacheFilesBorrowed {
            tree_data: &self.data,
            matrices_data: &self
                .matrices
                .data
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<u8>>(),
            matrices_palettes: &self.matrices.palettes.to_bytes(),
        };

        file_io.save_monster_matcher_cache_files(name, &tree_cache_files)
    }

    pub fn has_cache(file_io: &FileIo, name: &str) -> bool {
        file_io.has_monster_matcher_cache_folder(name)
    }

    fn load_from_cache(file_io: &FileIo, name: &str) -> io::Result<Self> {
        let tree_cache_files = file_io.load_monster_matcher_cache_files(name)?;

        let mut windows: Vec<[u8; WINDOW_SIZE]> =
            Vec::with_capacity(tree_cache_files.matrices_data.len() / WINDOW_SIZE);

        for chunk in tree_cache_files.matrices_data.chunks(WINDOW_SIZE) {
            let mut window = [0; WINDOW_SIZE];
            for (i, x) in chunk.iter().enumerate() {
                window[i] = *x;
            }
            windows.push(window);
        }

        let palettes = Palette::extract_palettes_from_bytes(&tree_cache_files.matrices_palettes);

        Ok(Self {
            data: tree_cache_files.tree_data,
            matrices: Matrices {
                data: windows,
                matrix_dimensions: PointU16 { row: 4, col: 4 },
                palettes: Palettes { data: palettes },
            },
            level: name.to_string(),
        })
    }

    fn handle_splits<C>(
        matrices: &Matrices,
        data: &mut Vec<u8>,
        point: u32,
        split: HashMap<u8, C, BuildNoHashHasher<u8>>,
    ) where
        C: Deref<Target = [MatrixPalette]>,
    {
        data.push(0);
        let num_values_index = data.len() - 1;

        let mut num_values = 0;

        for (k, v) in split.iter() {
            if !v.is_empty() {
                num_values += 1;

                data.push(*k);
            }
        }

        data[num_values_index] = num_values as u8;

        let mut next_node_pointer_offset = data.len() as u32;
        if num_values > 1 {
            for _ in 0..(num_values - 1) * 4 {
                data.push(0);
            }
        }

        let mut is_first_sub_node = true;

        for v in split.into_values() {
            if !v.is_empty() {
                let items = v.deref();

                let next_node_offset = Self::create_tree(matrices, data, items, point + 1);

                if is_first_sub_node {
                    is_first_sub_node = false;
                } else {
                    let next_node_pointer_bytes = next_node_offset.to_be_bytes();

                    data[next_node_pointer_offset as usize] = next_node_pointer_bytes[0];
                    data[next_node_pointer_offset as usize + 1] = next_node_pointer_bytes[1];
                    data[next_node_pointer_offset as usize + 2] = next_node_pointer_bytes[2];
                    data[next_node_pointer_offset as usize + 3] = next_node_pointer_bytes[3];

                    next_node_pointer_offset += 4;
                }
            }
        }
    }

    fn create_tree(
        matrices: &Matrices,
        data: &mut Vec<u8>,
        matrix_palettes: &[MatrixPalette],
        point: u32,
    ) -> u32 {
        let node_offset = data.len() as u32;

        if matrix_palettes.len() == 1 {
            data.push(0);
            data.extend(matrix_palettes[0].get_bytes());

            return node_offset;
        }

        if matrix_palettes.len() > 8 {
            let split = split_matrix_palettes_with_vec(matrices, matrix_palettes, point);

            Self::handle_splits(matrices, data, point, split);
        } else {
            let split = split_matrix_palettes_with_array_vec(matrices, matrix_palettes, point);
            Self::handle_splits(matrices, data, point, split);
        }

        node_offset
    }

    pub fn look_up(&self, img: &Matrix) -> Vec<TreeMatch> {
        let mut matches = Vec::new();
        let window_offset_points = img.get_window_offset_points(self.matrices.matrix_dimensions);
        let matrix_points = self.matrices.matrix_dimensions.get_points();

        let mut splits: Vec<[Vec<PointU16>; 256]> = Vec::new();

        const EMPTY_VEC: Vec<PointU16> = Vec::new();

        for _ in 0..17 {
            splits.push([EMPTY_VEC; 256]);
        }

        splits[0][0] = window_offset_points;

        self._look_up(img, &mut matches, &matrix_points, 0, &mut splits, 1, 0);

        matches
    }

    fn is_full_match(&self, img: &Matrix, tree_match: TreeMatch) -> bool {
        let mut c = 0;
        for row in 0..self.matrices.matrix_dimensions.row {
            for col in 0..self.matrices.matrix_dimensions.col {
                let img_value =
                    img.get_value(tree_match.window_offset_point + PointU16 { row, col });
                let matrix_value = self.matrices.get_value(tree_match.matrix_palette, c);

                if img_value != matrix_value {
                    return false;
                }

                c += 1;
            }
        }

        true
    }

    fn get_mask(&self, node_offset: usize, num_values: u8) -> [bool; 256] {
        let mut mask = [false; 256];

        for x in &self.data[node_offset..node_offset + num_values as usize] {
            mask[*x as usize] = true;
        }

        mask
    }

    fn _look_up(
        &self,
        img: &Matrix,
        matches: &mut Vec<TreeMatch>,
        matrix_points: &[PointU16],
        mut node_offset: usize,
        splits: &mut Vec<[Vec<PointU16>; 256]>,
        level: u8,
        value: u8,
    ) {
        let num_values = self.data[node_offset];
        node_offset += 1;
        if num_values == 0 {
            let matrix_palette = MatrixPalette::from_bytes(&self.data[node_offset..]);

            for window_offset_point in splits[level as usize - 1][value as usize].iter() {
                let tree_match = TreeMatch {
                    matrix_palette,
                    window_offset_point: *window_offset_point,
                };
                if self.is_full_match(img, tree_match) {
                    matches.push(tree_match);
                }
            }

            return;
        }

        let mask = self.get_mask(node_offset, num_values);

        split_window_offset_points(img, value, matrix_points[0], &mask, splits, level);

        let matrix_points = &matrix_points[1..];

        for i in 0..num_values as usize {
            let value = self.data[node_offset + i];

            if splits[level as usize][value as usize].is_empty() {
                continue;
            }

            let next_node_pointer: u32 = if i == 0 {
                (node_offset + num_values as usize + ((num_values as usize - 1) * 4)) as u32
            } else {
                let next_node_pointer_bytes_offset =
                    node_offset + num_values as usize + ((i - 1) * 4);
                let next_node_pointer_bytes: [u8; 4] = self.data
                    [next_node_pointer_bytes_offset..next_node_pointer_bytes_offset + 4]
                    .try_into()
                    .expect("slice with incorrect length");
                u32::from_be_bytes(next_node_pointer_bytes)
            };

            self._look_up(
                img,
                matches,
                matrix_points,
                next_node_pointer as usize,
                splits,
                level + 1,
                value,
            );

            splits[level as usize][value as usize].clear();
        }
    }
}

impl Palettes {
    fn get_value(&self, palette_index: u16, value_index: u8) -> u8 {
        self.data[palette_index as usize].transform(value_index)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.data.len() * 256);

        for palette in &self.data {
            bytes.extend(palette.palette);
        }

        bytes
    }
}

impl Matrices {
    fn new(data: Vec<[u8; WINDOW_SIZE]>, matrix_dimensions: PointU16, palettes: Palettes) -> Self {
        let mut matrices = Matrices {
            data,
            matrix_dimensions,
            palettes,
        };

        matrices.data.shrink_to_fit();
        matrices.palettes.data.shrink_to_fit();

        matrices
    }

    fn get_value(&self, index: MatrixPalette, point: u32) -> u8 {
        let matrix_value = self.data[index.matrix_id as usize][point as usize];
        self.palettes.get_value(index.palette_id, matrix_value)
    }
}

fn is_sprite_valid(data: [u8; WINDOW_SIZE]) -> bool {
    !has_zero_value(data) && count_unique_values(data) > 3
}

fn has_zero_value(data: [u8; WINDOW_SIZE]) -> bool {
    for x in data {
        if x == 0 || x == 172 {
            return true;
        }
    }

    false
}

fn count_unique_values(data: [u8; WINDOW_SIZE]) -> usize {
    let mut mask: [u8; 256] = [0; 256];

    let mut count = 0;
    for x in &data {
        if mask[*x as usize] == 0 {
            count += 1;
            mask[*x as usize] = 1;
        }
    }
    count
}

fn parse_dcc_into_sprite_window_structs(dcc: &Dcc) -> Vec<SpriteWindows> {
    let mut sprite_windows = Vec::new();

    let mut window = [0; WINDOW_SIZE];

    for direction in &dcc.directions {
        for frame in &direction.frames {
            let mut windows = Vec::new();

            let num_vertical_windows = (frame.dims.row / 4) as usize;
            let num_horizontal_windows = (frame.dims.col / 4) as usize;

            for row in 0..num_vertical_windows {
                for col in 0..num_horizontal_windows {
                    let mut i = 0;
                    let mut has_zero_value = false;
                    for r in 0..4 {
                        for c in 0..4 {
                            let frame_idx =
                                ((row * 4 + r) * frame.dims.col as usize) + (col * 4) + c;
                            let value = frame.data[frame_idx];

                            if value == 0 || value == 172 {
                                has_zero_value = true;
                                break;
                            }

                            window[i] = value;
                            i += 1;
                        }

                        if has_zero_value {
                            break;
                        }
                    }

                    if !has_zero_value {
                        let num_unique_values = count_unique_values(window);
                        if num_unique_values > 3 {
                            let sprite_window = SpriteWindow {
                                data: window,
                                num_unique_values: num_unique_values as u32,
                            };
                            windows.push(sprite_window)
                        }
                    }
                }
            }

            windows.sort_by_key(|window| window.num_unique_values);
            windows.reverse();

            sprite_windows.push(SpriteWindows { windows })
        }
    }

    sprite_windows
}

fn transform_window(window: &[u8; WINDOW_SIZE], palette: &Palette) -> [u8; WINDOW_SIZE] {
    let mut transformed_window = [0; WINDOW_SIZE];

    for i in 0..WINDOW_SIZE {
        transformed_window[i] = palette.transform(window[i]);
    }

    transformed_window
}

fn load_palettes(
    palettes: &mut Vec<Palette>,
    palshift_palettes: &Option<Vec<Palette>>,
    neutral_palette: &Palette,
    num_light_radius_palettes: usize,
    palshift_id: u8,
) -> (usize, usize) {
    let mut palette_start_id = 0;
    let mut palette_end_id = 1 + num_light_radius_palettes;

    if let Some(palshift_palettes) = palshift_palettes {
        let palshift_palette = &palshift_palettes[2 + palshift_id as usize];

        if palshift_palette != neutral_palette {
            palette_start_id = palettes.len();
            palettes.push(palettes[0].combine_palettes(palshift_palette));

            for i in 0..num_light_radius_palettes {
                palettes.push(palettes[i + 1].combine_palettes(palshift_palette));
            }
            palette_end_id = palettes.len();
        }
    }

    (palette_start_id, palette_end_id)
}

struct SpriteWindowSetup<'a> {
    matrix_windows_neutral: &'a mut Vec<[u8; 16]>,
    window_hashset: &'a mut FastHashSet<[u8; WINDOW_SIZE]>,
    filename_to_window_ids: &'a mut HashMap<String, (usize, usize)>,
}

fn load_window_sprites(
    archives: &mut Archives,
    sprite_window_setup: &mut SpriteWindowSetup,
    dcc_file: &DccFile,
    max_windows_per_sprite_frame: u32,
) {
    match sprite_window_setup
        .filename_to_window_ids
        .get(&dcc_file.name)
    {
        Some(_) => {}
        None => {
            if let Ok(dcc_file_bytes) = archives.extract_dcc_file_bytes(&dcc_file.full_path) {
                let start = sprite_window_setup.matrix_windows_neutral.len();

                let dcc = dcc_file_bytes.parse();

                let sprites_windows = parse_dcc_into_sprite_window_structs(&dcc);

                for sprite_windows in &sprites_windows {
                    let mut window_count = 0;
                    for window in &sprite_windows.windows {
                        if sprite_window_setup.window_hashset.contains(&window.data) {
                            continue;
                        }

                        sprite_window_setup.matrix_windows_neutral.push(window.data);

                        window_count += 1;
                        if window_count == max_windows_per_sprite_frame {
                            break;
                        }
                    }
                }

                let end = sprite_window_setup.matrix_windows_neutral.len();
                sprite_window_setup
                    .filename_to_window_ids
                    .insert(dcc_file.name.clone(), (start, end));
            }
        }
    }
}

fn add_sprite_windows(
    sprite_window_setup: &mut SpriteWindowSetup,
    matrix_palettes: &mut Vec<MatrixPalette>,
    processed_data: &mut HashSet<(usize, usize, usize, usize)>,
    palettes: &[Palette],
    dcc_file: &DccFile,
    palette_start_id: usize,
    palette_end_id: usize,
) {
    if let Some((start, end)) = sprite_window_setup
        .filename_to_window_ids
        .get(&dcc_file.name)
    {
        if processed_data.contains(&(*start, *end, palette_start_id, palette_end_id)) {
            return;
        }
        for (window_id, window) in sprite_window_setup
            .matrix_windows_neutral
            .iter()
            .enumerate()
            .take(*end)
            .skip(*start)
        {
            for (palette_id, palette) in palettes
                .iter()
                .enumerate()
                .take(palette_end_id)
                .skip(palette_start_id)
            {
                let trasformed_window = transform_window(window, palette);

                if is_sprite_valid(trasformed_window) {
                    let new_window = sprite_window_setup
                        .window_hashset
                        .insert(&trasformed_window);

                    if new_window {
                        matrix_palettes.push(MatrixPalette {
                            matrix_id: window_id as u32,
                            palette_id: palette_id as u16,
                        });
                    }
                }
            }
        }
    }
}

fn get_level_monsters(
    level: LevelName,
    game_difficulty: GameDifficulty,
    archives: &mut Archives,
) -> Result<Vec<Monster>, BotError> {
    let excel_levels_raw_text = archives.extract_excel_levels_raw_text()?;
    let excel_levels = excel_levels_raw_text.parse();

    let excel_monstats_raw_text = archives.extract_excel_monstats_raw_text()?;
    let excel_monstats = excel_monstats_raw_text.parse();

    let excel_monstats2_raw_text = archives.extract_excel_monstats2_raw_text()?;
    let excel_monstats2 = excel_monstats2_raw_text.parse();

    Ok(get_monsters_in_level(
        level,
        &excel_levels,
        &excel_monstats,
        &excel_monstats2,
        game_difficulty,
    ))
}

fn get_monster_data(name: &str, archives: &mut Archives) -> Monster {
    let excel_monstats_raw_text = archives.extract_excel_monstats_raw_text().unwrap();
    let excel_monstats = excel_monstats_raw_text.parse();

    let excel_monstats2_raw_text = archives.extract_excel_monstats2_raw_text().unwrap();
    let excel_monstats2 = excel_monstats2_raw_text.parse();

    get_monster(&excel_monstats, &excel_monstats2, name)
}

fn group_monsters(monsters: Vec<Monster>) -> HashMap<String, Vec<Monster>> {
    let mut monster_groups = HashMap::new();

    for monster in monsters.into_iter() {
        let code = monster.code.clone();
        monster_groups
            .entry(code)
            .or_insert_with(Vec::new)
            .push(monster);
    }

    monster_groups
}

pub fn cache_monster_tree(
    archives: &mut Archives,
    file_io: &FileIo,
    name: &str,
    config: &MonsterMatcherConfig,
    zone_name_converter: &ZoneNameConverter,
) {
    if Tree::has_cache(file_io, name) {
        return;
    }

    let _monster_tree = get_monster_tree(archives, file_io, name, config, &zone_name_converter);
}

pub struct MonsterMatcherConfig {
    pub act: Act,
    pub game_difficulty: GameDifficulty,
    pub match_unique_and_champion_monsters: bool,
    pub max_windows_per_sprite_frame: u32,
    pub monster_matcher_type: MonsterMatcherType,
}

impl MonsterMatcherConfig {
    pub fn new_npc_matcher_config(act: Act, game_difficulty: GameDifficulty) -> Self {
        Self {
            act,
            game_difficulty,
            match_unique_and_champion_monsters: false,
            max_windows_per_sprite_frame: 12,
            monster_matcher_type: MonsterMatcherType::Monster,
        }
    }

    pub fn new_levels_monster_matcher_config(
        act: Act,
        game_difficulty: GameDifficulty,
        bot_settings: &BotSettings,
    ) -> Self {
        Self {
            act,
            game_difficulty,
            match_unique_and_champion_monsters: bot_settings.match_unique_and_champion_monsters,
            max_windows_per_sprite_frame: bot_settings.max_windows_per_sprite_frame,
            monster_matcher_type: MonsterMatcherType::Level,
        }
    }
}

pub fn get_monster_tree(
    archives: &mut Archives,
    file_io: &FileIo,
    name: &str,
    config: &MonsterMatcherConfig,
    zone_name_converter: &ZoneNameConverter,
) -> io::Result<Tree> {
    if Tree::has_cache(file_io, name) {
        return Tree::load_from_cache(file_io, name);
    }

    let monsters = match config.monster_matcher_type {
        MonsterMatcherType::Level => get_level_monsters(
            LevelName(zone_name_converter.get_default_level_name_from_english_level_name(name)),
            config.game_difficulty,
            archives,
        )
        .unwrap(),
        MonsterMatcherType::Monster => {
            vec![get_monster_data(name, archives)]
        }
    };

    let light_radius_palettes = archives
        .extract_pal_pl2_bytes(config.act.into())
        .unwrap()
        .extract_light_radius_palette_bytes()
        .get_palettes();

    let mut palettes: Vec<Palette> = Vec::new();

    let neutral_palette = Palette::get_neutral_palette();
    let num_light_radius_palettes = light_radius_palettes.len();

    let rand_transform_palettes = archives.extract_rand_transform_palettes().unwrap().parse();

    palettes.push(neutral_palette);
    palettes.extend(light_radius_palettes);

    let rand_transform_palettes_start_id = palettes.len();

    if config.monster_matcher_type == MonsterMatcherType::Level
        && config.match_unique_and_champion_monsters
    {
        let rand_transform_light_radius_combined_palettes =
            Palette::combine_multiple_palettes(&palettes, &rand_transform_palettes.palettes);

        palettes.extend(rand_transform_light_radius_combined_palettes);
    }

    let rand_transform_palettes_end_id = palettes.len();

    let neutral_palette = Palette::get_neutral_palette();

    let mut matrix_windows_neutral: Vec<[u8; 16]> = Vec::new();
    let mut matrix_palettes: Vec<MatrixPalette> = Vec::new();
    let mut window_hashset: FastHashSet<[u8; WINDOW_SIZE]> = FastHashSet::new();

    let mut filename_to_window_ids: HashMap<String, (usize, usize)> = HashMap::new();
    let mut processed_data: HashSet<(usize, usize, usize, usize)> = HashSet::new();

    let mut sprite_window_setup = SpriteWindowSetup {
        filename_to_window_ids: &mut filename_to_window_ids,
        matrix_windows_neutral: &mut matrix_windows_neutral,
        window_hashset: &mut window_hashset,
    };

    let now = Instant::now();

    let monster_groups = group_monsters(monsters);

    for (monster_code, monsters) in monster_groups.iter() {
        let palshift_palettes = match archives.extract_palshift_palettes_bytes(monster_code) {
            Ok(bytes) => Some(Palette::extract_palettes_from_bytes(&bytes)),
            Err(_) => None,
        };

        for monster in monsters {
            println!("\t{}", monster.name);
            let dcc_files = monster.get_dcc_file_paths();

            let (palette_start_id, palette_end_id) = load_palettes(
                &mut palettes,
                &palshift_palettes,
                &neutral_palette,
                num_light_radius_palettes,
                monster.palshift_id,
            );

            for dcc_file in dcc_files {
                load_window_sprites(
                    archives,
                    &mut sprite_window_setup,
                    &dcc_file,
                    config.max_windows_per_sprite_frame,
                );
                add_sprite_windows(
                    &mut sprite_window_setup,
                    &mut matrix_palettes,
                    &mut processed_data,
                    &palettes,
                    &dcc_file,
                    palette_start_id,
                    palette_end_id,
                );

                if config.monster_matcher_type == MonsterMatcherType::Level {
                    add_sprite_windows(
                        &mut sprite_window_setup,
                        &mut matrix_palettes,
                        &mut processed_data,
                        &palettes,
                        &dcc_file,
                        rand_transform_palettes_start_id,
                        rand_transform_palettes_end_id,
                    );
                }
            }
        }
    }

    println!("elapsed: {:?}", now.elapsed());
    println!("matrix_palettes.len(): {}", matrix_palettes.len());
    println!("palettes.len(): {}", palettes.len());

    let matrices = Matrices::new(
        matrix_windows_neutral,
        PointU16 { row: 4, col: 4 },
        Palettes { data: palettes },
    );

    let now = Instant::now();
    let tree = Tree::new(matrices, &matrix_palettes, name.to_string());
    println!("elapsed create_tree: {} millis", now.elapsed().as_millis());

    tree.save_to_cache(file_io, name)?;

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        box_u16::BoxU16,
        enums::{
            act::Act, game_difficulty::GameDifficulty, monster_matcher_type::MonsterMatcherType,
        },
        file_io::FileIo,
        image::Image,
        matrix::Matrix,
        mpq_archives::archives::Archives,
        pattern_matcher_monsters::MonsterMatcherConfig,
        point_u16::PointU16,
        string_tables::{StringTables, ZoneNameConverter},
    };

    use super::get_monster_tree;

    impl Matrix {
        fn draw_border(&mut self, border: BoxU16) {
            const COLOR: u8 = 255;

            // Draw top and bottom
            for col in border.offset.col..border.offset.col + border.dimensions.col {
                self.set_value(
                    PointU16 {
                        row: border.offset.row,
                        col,
                    },
                    COLOR,
                );
                self.set_value(
                    PointU16 {
                        row: border.offset.row + border.dimensions.row - 1,
                        col,
                    },
                    COLOR,
                );
            }

            // Draw left and right
            for row in border.offset.row..border.offset.row + border.dimensions.row {
                self.set_value(
                    PointU16 {
                        row,
                        col: border.offset.col,
                    },
                    COLOR,
                );
                self.set_value(
                    PointU16 {
                        row,
                        col: border.offset.col + border.dimensions.col - 1,
                    },
                    COLOR,
                );
            }
        }
    }

    #[test]
    fn test_pattern_matcher_npc() {
        let file_io = FileIo::new();
        file_io.create_temp_dir().unwrap();
        let system_settings = file_io.load_system_settings().unwrap();
        let act = Act::Act4;

        let mut archives = Archives::new(&system_settings.diablo2_folder_path);
        let npc_name = &act.get_potion_seller_name();

        let string_tables = StringTables::new(&mut archives);
        let zone_name_converter = ZoneNameConverter::new(&string_tables);

        let monster_matcher_config = MonsterMatcherConfig {
            act,
            game_difficulty: GameDifficulty::Hell,
            match_unique_and_champion_monsters: false,
            max_windows_per_sprite_frame: 12,
            monster_matcher_type: MonsterMatcherType::Monster,
        };

        let tree = get_monster_tree(
            &mut archives,
            &file_io,
            npc_name,
            &monster_matcher_config,
            &zone_name_converter,
        )
        .unwrap();

        let img_path = file_io
            .root
            .join("test_data")
            .join("monsters")
            .join("npcs")
            .join(npc_name)
            .join("0.png");

        let img = Image::load_image(&img_path);

        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

        let mut matrix = img.to_matrix(&palette_transformer);

        let now = std::time::Instant::now();
        let mut matches = tree.look_up(&matrix);
        println!("Time look_up: {:?}", now.elapsed());
        println!("matches.len(): {}", matches.len());
        matches.sort_by(|a, b| a.window_offset_point.cmp(&b.window_offset_point));

        for m in matches.iter() {
            matrix.draw_border(BoxU16 {
                offset: m.window_offset_point,
                dimensions: PointU16::new(4, 4),
            });
        }

        matrix
            .to_image(&pixel_palette)
            .save_image(
                &Path::new(&file_io.root)
                    .join("temp")
                    .join("pattern_matcher_npc.png"),
            )
            .unwrap();
    }

    #[test]
    fn test_pattern_matcher_monsters() {
        let file_io = FileIo::new();
        file_io.create_temp_dir().unwrap();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let string_tables = StringTables::new(&mut archives);
        let zone_name_converter = ZoneNameConverter::new(&string_tables);

        let act = Act::Act1;

        let monster_matcher_config = MonsterMatcherConfig {
            act,
            game_difficulty: GameDifficulty::Hell,
            match_unique_and_champion_monsters: true,
            max_windows_per_sprite_frame: 2,
            monster_matcher_type: MonsterMatcherType::Level,
        };

        let tree = get_monster_tree(
            &mut archives,
            &file_io,
            "Stony Field",
            &monster_matcher_config,
            &zone_name_converter,
        )
        .unwrap();

        let img_path = file_io
            .root
            .join("test_data")
            .join("monsters")
            .join("palette")
            .join("ACT1")
            .join("0")
            .join("monsters.png");

        let img = Image::load_image(&img_path);

        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

        let mut matrix = img.to_matrix(&palette_transformer);

        let now = std::time::Instant::now();
        let mut matches = tree.look_up(&matrix);
        println!("Time look_up: {:?}", now.elapsed());
        println!("matches.len(): {}", matches.len());
        matches.sort_by(|a, b| a.window_offset_point.cmp(&b.window_offset_point));

        for m in matches.iter() {
            matrix.draw_border(BoxU16 {
                offset: m.window_offset_point,
                dimensions: PointU16::new(4, 4),
            });
        }

        matrix
            .to_image(&pixel_palette)
            .save_image(
                &Path::new(&file_io.root)
                    .join("temp")
                    .join("pattern_matcher_monsters.png"),
            )
            .unwrap();
    }
}
