// http://classic.battle.net/diablo2exp/items/potions.shtml

use crate::{
    constants::{belt_items::BELT_ITEMS, table_meta_data::BELT_TABLE_META_DATA},
    enums::{
        belt_item_type::{BeltItemType, HealthManaPotionType},
        character_class::CharacterClass,
        errors::LowHealthManaAndNoPotionInBeltError,
        potion_points_type::PotionPointsType,
    },
    health_mana::Points,
    matrix::Matrix,
    output_controller::OutputController,
    point_u16::PointU16,
    spell_caster::get_current_time_milliseconds,
    table::TableMetaData,
    table_matcher::ConsumableItemsTableMatcher,
    units::Frames,
    utils::{sleep_frame, sleep_frames},
};

pub struct BeltItem {
    pub name: &'static str,
    pub inventory_sprite_file_name: &'static str,
    pub(crate) belt_item_type: BeltItemType,
}

fn get_belt_item_type_from_name(item_name: &str) -> Option<BeltItemType> {
    for belt_item in &BELT_ITEMS {
        if belt_item.name == item_name {
            return Some(belt_item.belt_item_type);
        }
    }

    None
}

pub struct Belt {
    items: Vec<Vec<Option<BeltItemType>>>,
    table_meta_data: TableMetaData,
    num_belt_columns_reserved_for_healing_potions: u8,
    num_belt_columns_reserved_for_mana_potions: u8,
    healing_potion_last_consumed_time_milliseconds: u64,
    mana_potion_last_consumed_time_milliseconds: u64,
}

impl Belt {
    pub fn new(
        num_belt_columns_reserved_for_healing_potions: u8,
        num_belt_columns_reserved_for_mana_potions: u8,
    ) -> Self {
        let table_meta_data: TableMetaData = BELT_TABLE_META_DATA;

        let items = vec![
            vec![None; table_meta_data.table_size.col as usize];
            table_meta_data.table_size.row as usize
        ];

        Self {
            items,
            table_meta_data,
            num_belt_columns_reserved_for_healing_potions,
            num_belt_columns_reserved_for_mana_potions,
            healing_potion_last_consumed_time_milliseconds: 0,
            mana_potion_last_consumed_time_milliseconds: 0,
        }
    }

    pub fn remove_unneeded_potions(&mut self, output_controller: &mut OutputController) {
        for _ in 0..self.table_meta_data.table_size.row as usize {
            if !self._remove_unneeded_potions(output_controller) {
                return;
            } else {
                sleep_frames(Frames(4));
            }
        }
    }

    fn _remove_unneeded_potions(&mut self, output_controller: &mut OutputController) -> bool {
        let columns_with_healing_potions =
            self.get_columns_of_type(PotionPointsType::Health, false);

        let columns_with_mana_potions = self.get_columns_of_type(PotionPointsType::Mana, false);

        let mut found_unneeded_potions = false;

        if columns_with_healing_potions.len() as u8
            > self.num_belt_columns_reserved_for_healing_potions
        {
            found_unneeded_potions = true;

            let diff = columns_with_healing_potions.len() as u8
                - self.num_belt_columns_reserved_for_healing_potions;

            for col_id in columns_with_healing_potions[..diff as usize].iter() {
                self.drink_potion_in_column(output_controller, *col_id as usize);
                sleep_frame();
            }
        }

        if columns_with_mana_potions.len() as u8 > self.num_belt_columns_reserved_for_mana_potions {
            found_unneeded_potions = true;

            let diff = columns_with_mana_potions.len() as u8
                - self.num_belt_columns_reserved_for_mana_potions;

            for col_id in columns_with_mana_potions[..diff as usize].iter() {
                self.drink_potion_in_column(output_controller, *col_id as usize);
                sleep_frame();
            }
        }

        found_unneeded_potions
    }

    pub fn reset_potion_consume_time(&mut self) {
        self.healing_potion_last_consumed_time_milliseconds = 0;
        self.mana_potion_last_consumed_time_milliseconds = 0;
    }

    fn has_mana_potion(&self) -> bool {
        let row = self.items.len() - 1;

        for col in 0..4 {
            let potion = self.items[row][col];

            if let Some(BeltItemType::ManaPotion(_)) = potion {
                return true;
            }
        }

        false
    }

    pub fn get_id_of_column_with_optimal_potion(
        &self,
        class: CharacterClass,
        points: Points,
        potion_points_type: PotionPointsType,
    ) -> Option<usize> {
        let cols = self.table_meta_data.table_size.col as usize;
        let row_id = self.items.len() - 1;

        let missing_points = points.max - points.current;

        let has_mana_potion = self.has_mana_potion();

        (0..cols)
            .filter_map(|col_id| {
                let potion = self.items[row_id][col_id];

                let points = match potion_points_type {
                    PotionPointsType::Health => match potion {
                        Some(BeltItemType::HealingPotion(healing_potion_type)) => {
                            Some(healing_potion_type.get_points(class))
                        }
                        Some(BeltItemType::RejuvenationPotion(rejuvenation_potion_type)) => {
                            Some(rejuvenation_potion_type.get_points(points.max))
                        }
                        _ => None,
                    },
                    PotionPointsType::Mana => match potion {
                        Some(BeltItemType::ManaPotion(mana_potion_type)) => {
                            Some(mana_potion_type.get_points(class))
                        }
                        Some(BeltItemType::RejuvenationPotion(rejuvenation_potion_type)) => {
                            match has_mana_potion {
                                true => None,
                                false => Some(rejuvenation_potion_type.get_points(points.max)),
                            }
                        }
                        _ => None,
                    },
                };

                if let Some(points) = points {
                    let distance_to_max_points =
                        i32::abs(missing_points as i32 - points as i32) as u32;
                    Some((col_id, distance_to_max_points))
                } else {
                    None
                }
            })
            .min_by_key(|x| x.1)
            .map(|x| x.0)
    }

    pub fn update_belt_and_remove_unneeded_potions(
        &mut self,
        matrix: &Matrix,
        table_matcher: &ConsumableItemsTableMatcher,
        output_controller: &mut OutputController,
    ) {
        self.update_belt(matrix, table_matcher);
        self.remove_unneeded_potions(output_controller);
    }

    pub fn update_belt(&mut self, matrix: &Matrix, table_matcher: &ConsumableItemsTableMatcher) {
        self.clear();

        let table = table_matcher.match_from_matrix(matrix, self.table_meta_data);

        for (row_id, row) in table.cells.iter().enumerate() {
            for (col_id, col) in row.iter().enumerate() {
                self.items[row_id][col_id] = match &col {
                    Some(item_name) => get_belt_item_type_from_name(item_name),
                    None => None,
                };
            }
        }

        let is_belt_valid = self.is_belt_valid();

        if !is_belt_valid {
            println!("Belt is invalid");
        }
    }

    fn drink_potion_in_column(
        &mut self,
        output_controller: &mut OutputController,
        col_id: usize,
    ) -> Option<HealthManaPotionType> {
        let keyboard_char = match col_id {
            0 => '1',
            1 => '2',
            2 => '3',
            _ => '4',
        };

        output_controller.click_key(enigo::Key::Layout(keyboard_char));

        self.consume_item(col_id)
    }

    fn drink_potion_of_type(
        &mut self,
        output_controller: &mut OutputController,
        class: CharacterClass,
        points: Points,
        potion_type: PotionPointsType,
    ) -> Result<Option<HealthManaPotionType>, LowHealthManaAndNoPotionInBeltError> {
        let optimal_potion_column =
            self.get_id_of_column_with_optimal_potion(class, points, potion_type);

        match optimal_potion_column {
            Some(col_id) => Ok(self.drink_potion_in_column(output_controller, col_id)),
            None => {
                return Err(LowHealthManaAndNoPotionInBeltError);
            }
        }
    }

    pub fn drink_potions_if_points_under_soft_limit(
        &mut self,
        output_controller: &mut OutputController,
        class: CharacterClass,
        points: Points,
        limit: f32,
        potion_type: PotionPointsType,
    ) -> Result<Option<HealthManaPotionType>, LowHealthManaAndNoPotionInBeltError> {
        let ratio = points.current as f32 / points.max as f32;
        let under_limit = ratio < limit;

        let time_since_last_consume_milliseconds = get_current_time_milliseconds()
            - match potion_type {
                PotionPointsType::Health => self.healing_potion_last_consumed_time_milliseconds,
                PotionPointsType::Mana => self.mana_potion_last_consumed_time_milliseconds,
            };

        let potion_fill_time_milliseconds = match potion_type {
            PotionPointsType::Health => 7000,
            PotionPointsType::Mana => 5000,
        };

        if under_limit && time_since_last_consume_milliseconds > potion_fill_time_milliseconds {
            return self.drink_potion_of_type(output_controller, class, points, potion_type);
        }

        Ok(None)
    }

    fn consume_item(&mut self, col_id: usize) -> Option<HealthManaPotionType> {
        if col_id >= self.table_meta_data.table_size.col as usize {
            return None;
        }

        let potion_type = if let Some(potion_type) = self.items[self.items.len() - 1][col_id] {
            match potion_type {
                BeltItemType::HealingPotion(_) => Some(HealthManaPotionType::HealingPotion),
                BeltItemType::ManaPotion(_) => Some(HealthManaPotionType::ManaPotion),
                BeltItemType::RejuvenationPotion(_) => {
                    Some(HealthManaPotionType::RejuvenationPotion)
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(potion_type) = self.items[self.items.len() - 1][col_id] {
            match potion_type {
                BeltItemType::HealingPotion(_) => {
                    self.healing_potion_last_consumed_time_milliseconds =
                        get_current_time_milliseconds();
                }
                BeltItemType::ManaPotion(_) => {
                    self.mana_potion_last_consumed_time_milliseconds =
                        get_current_time_milliseconds();
                }
                _ => {}
            }
        }

        for row_id in (1..self.items.len()).rev() {
            self.items[row_id][col_id] = self.items[row_id - 1][col_id].take();
        }

        self.items[0][col_id] = None;

        potion_type
    }

    pub fn clear(&mut self) {
        for row in &mut self.items {
            for col in row.iter_mut() {
                *col = None;
            }
        }
    }

    pub fn is_belt_valid(&self) -> bool {
        let cols = self.table_meta_data.table_size.col as usize;

        for col_id in 0..cols {
            let mut empty_found = false;

            // Check each row in the column from bottom to top
            for row_id in (0..self.items.len()).rev() {
                match self.items[row_id][col_id] {
                    None => empty_found = true,
                    Some(_) if empty_found => return false, // Invalid: found an item above an empty cell
                    _ => (),
                }
            }
        }

        true
    }

    pub fn auto_add_item(&mut self, item_name: &str) -> bool {
        let belt_item_type = get_belt_item_type_from_name(item_name);

        if let Some(belt_item_type) = belt_item_type {
            if belt_item_type.is_auto_pickup() {
                if let Some(cell_point) = self
                    .get_row_and_col_point_of_first_non_full_column_with_equal_item_type(
                        belt_item_type,
                    )
                {
                    self.items[cell_point.row as usize][cell_point.col as usize] =
                        Some(belt_item_type);
                    return true;
                } else {
                    let last_row_id = self.items.len() - 1;
                    match belt_item_type {
                        BeltItemType::HealingPotion(_) => {
                            let num_columns_of_type =
                                self.count_columns_of_type(PotionPointsType::Health, true);
                            if num_columns_of_type
                                < self.num_belt_columns_reserved_for_healing_potions
                            {
                                if let Some(col_id) = self.get_id_of_first_fully_empty_column() {
                                    self.items[last_row_id][col_id] = Some(belt_item_type);

                                    return true;
                                }
                            }
                        }
                        BeltItemType::ManaPotion(_) => {
                            let num_columns_of_type =
                                self.count_columns_of_type(PotionPointsType::Mana, true);
                            if num_columns_of_type < self.num_belt_columns_reserved_for_mana_potions
                            {
                                if let Some(col_id) = self.get_id_of_first_fully_empty_column() {
                                    self.items[last_row_id][col_id] = Some(belt_item_type);

                                    return true;
                                }
                            }
                        }
                        BeltItemType::RejuvenationPotion(_) => {
                            if let Some(col_id) = self.get_id_of_first_fully_empty_column() {
                                self.items[last_row_id][col_id] = Some(belt_item_type);

                                return true;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        false
    }

    fn get_columns_of_type(
        &self,
        potion_points_type: PotionPointsType,
        count_rejuvenation_potion_as_healing_and_mana: bool,
    ) -> Vec<u8> {
        let cols = self.table_meta_data.table_size.col as usize;
        let rows = self.table_meta_data.table_size.row as usize;

        let mut columns = Vec::new();

        for col_id in 0..cols {
            if let Some(item) = self.items[rows - 1][col_id] {
                match (
                    &potion_points_type,
                    count_rejuvenation_potion_as_healing_and_mana,
                ) {
                    (PotionPointsType::Health, true) => match item {
                        BeltItemType::HealingPotion(_) | BeltItemType::RejuvenationPotion(_) => {
                            columns.push(col_id as u8);
                        }
                        _ => (),
                    },
                    (PotionPointsType::Mana, true) => match item {
                        BeltItemType::ManaPotion(_) | BeltItemType::RejuvenationPotion(_) => {
                            columns.push(col_id as u8);
                        }
                        _ => (),
                    },
                    (PotionPointsType::Health, false) => match item {
                        BeltItemType::HealingPotion(_) => {
                            columns.push(col_id as u8);
                        }
                        _ => (),
                    },
                    (PotionPointsType::Mana, false) => match item {
                        BeltItemType::ManaPotion(_) => {
                            columns.push(col_id as u8);
                        }
                        _ => (),
                    },
                }
            }
        }

        columns
    }

    fn count_columns_of_type(
        &self,
        potion_points_type: PotionPointsType,
        count_rejuvenation_potion_as_healing_and_mana: bool,
    ) -> u8 {
        self.get_columns_of_type(
            potion_points_type,
            count_rejuvenation_potion_as_healing_and_mana,
        )
        .len() as u8
    }

    fn get_id_of_first_fully_empty_column(&self) -> Option<usize> {
        for col_id in 0..self.items[0].len() {
            let mut is_empty = true;

            for row in &self.items {
                if row[col_id].is_some() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                return Some(col_id);
            }
        }

        None
    }

    fn get_row_and_col_point_of_first_non_full_column_with_equal_item_type(
        &self,
        item_type: BeltItemType,
    ) -> Option<PointU16> {
        let cols = self.table_meta_data.table_size.col as usize;

        for col_id in 0..cols {
            let mut all_match_or_empty = true;
            let mut item_found = false;
            let mut empty_found = false;
            let mut last_empty_row_id = 0;

            for (row_id, row) in self.items.iter().enumerate() {
                match &row[col_id] {
                    Some(current_item) if item_type.is_equal_type(*current_item) => {
                        item_found = true;
                    }
                    Some(_) => {
                        all_match_or_empty = false;
                        break;
                    }
                    None => {
                        empty_found = true;
                        last_empty_row_id = row_id;
                    }
                }
            }

            if all_match_or_empty && item_found && empty_found {
                return Some(PointU16::new(last_empty_row_id as u16, col_id as u16));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    use crate::{
        belt::PotionPointsType,
        constants::belt_item_names::{
            ANTIDOTE_POTION, FULL_REJUVENATION_POTION, GREATER_HEALING_POTION, GREATER_MANA_POTION,
            HEALING_POTION, LIGHT_HEALING_POTION, LIGHT_MANA_POTION, MANA_POTION,
            MINOR_HEALING_POTION, MINOR_MANA_POTION, REJUVENATION_POTION, SCROLL_OF_IDENTIFY,
            SCROLL_OF_TOWN_PORTAL, STAMINA_POTION, SUPER_HEALING_POTION, SUPER_MANA_POTION,
            THAWING_POTION,
        },
        enums::{
            act::Act, belt_item_type::BeltItemType, character_class::CharacterClass,
            healing_potion_type::HealingPotionType, mana_potion_type::ManaPotionType,
            rejuvenation_potion_type::RejuvenationPotionType,
        },
        file_io::FileIo,
        font_char_map::get_non_control_ascii_char_font_map,
        health_mana::Points,
        image::Image,
        mpq_archives::archives::Archives,
        pal_pl2::PaletteTransformer,
        table_matcher::ConsumableItemsTableMatcher,
    };

    use super::Belt;

    #[test]
    fn test_belt() {
        env::set_var("RUST_BACKTRACE", "1");
        let file_io = FileIo::new();

        let mut belt = Belt::new(2, 2);

        test_belt_is_valid(&mut belt);
        test_auto_add_item(&mut belt);
        test_belt_consume_item(&mut belt);
        test_update_belt(&mut belt, &file_io);
        test_get_id_of_column_with_most_efficient_potion(&mut belt);
    }

    fn test_get_id_of_column_with_most_efficient_potion(belt: &mut Belt) {
        belt.clear();

        let num_rows = belt.table_meta_data.table_size.row as usize;

        belt.items[num_rows - 1][0] = Some(BeltItemType::HealingPotion(HealingPotionType::Minor));
        belt.items[num_rows - 1][1] = Some(BeltItemType::RejuvenationPotion(
            RejuvenationPotionType::FullRejuvenation,
        ));
        belt.items[num_rows - 1][2] = Some(BeltItemType::HealingPotion(HealingPotionType::Greater));
        belt.items[num_rows - 1][3] = Some(BeltItemType::RejuvenationPotion(
            RejuvenationPotionType::Rejuvenation,
        ));

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 900,
                    max: 1000
                },
                PotionPointsType::Health
            )
            .unwrap(),
            0
        );
        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 800,
                    max: 1000
                },
                PotionPointsType::Health
            )
            .unwrap(),
            2
        );
        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 700,
                    max: 1000
                },
                PotionPointsType::Health
            )
            .unwrap(),
            3
        );
        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 300,
                    max: 1000
                },
                PotionPointsType::Health
            )
            .unwrap(),
            1
        );

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 800,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            3
        );
        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 100,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            1
        );

        belt.items[num_rows - 1][0] = Some(BeltItemType::ManaPotion(ManaPotionType::Minor));
        belt.items[num_rows - 1][1] = Some(BeltItemType::RejuvenationPotion(
            RejuvenationPotionType::FullRejuvenation,
        ));
        belt.items[num_rows - 1][2] = Some(BeltItemType::ManaPotion(ManaPotionType::Greater));
        belt.items[num_rows - 1][3] = Some(BeltItemType::RejuvenationPotion(
            RejuvenationPotionType::Rejuvenation,
        ));

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 900,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            0
        );
        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 800,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            2
        );

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 300,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            2
        );

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 900,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            0
        );

        belt.items[num_rows - 1][0] = None;
        belt.items[num_rows - 1][2] = None;

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 200,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            1
        );

        assert_eq!(
            belt.get_id_of_column_with_optimal_potion(
                CharacterClass::Druid,
                Points {
                    current: 660,
                    max: 1000
                },
                PotionPointsType::Mana
            )
            .unwrap(),
            3
        );
    }

    fn test_update_belt(belt: &mut Belt, file_io: &FileIo) {
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
            ConsumableItemsTableMatcher::new(&mut archives, &font_char_map);

        let image_path = file_io
            .root
            .join("test_data")
            .join("table")
            .join("filled_belt")
            .join("items.png");

        let expected_belt = vec![
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Minor)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::StaminaPotion),
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Light)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Light)),
                Some(BeltItemType::HealingPotion(HealingPotionType::Super)),
                Some(BeltItemType::ThawingPotion),
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Standard)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Standard)),
                Some(BeltItemType::RejuvenationPotion(
                    RejuvenationPotionType::Rejuvenation,
                )),
                Some(BeltItemType::AntidotePotion),
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Greater)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Greater)),
                Some(BeltItemType::RejuvenationPotion(
                    RejuvenationPotionType::FullRejuvenation,
                )),
                Some(BeltItemType::ScrollOfIdentify),
            ],
        ];

        test_update_belt_full_belt(
            belt,
            &image_path,
            &consumable_item_matcher,
            &palette_transformer,
            expected_belt,
        );

        let image_path = file_io
            .root
            .join("test_data")
            .join("table")
            .join("partially_filled_belt")
            .join("items.png");

        let expected_belt = vec![
            vec![None, None, None, None],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                None,
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
            ],
        ];

        test_update_belt_full_belt(
            belt,
            &image_path,
            &consumable_item_matcher,
            &palette_transformer,
            expected_belt,
        );
    }

    fn test_auto_add_item(belt: &mut Belt) {
        belt.clear();

        belt.auto_add_item(STAMINA_POTION);
        belt.auto_add_item(ANTIDOTE_POTION);
        belt.auto_add_item(THAWING_POTION);
        belt.auto_add_item(SCROLL_OF_TOWN_PORTAL);
        belt.auto_add_item(SCROLL_OF_IDENTIFY);

        belt.auto_add_item(MINOR_HEALING_POTION);
        belt.auto_add_item(LIGHT_HEALING_POTION);
        belt.auto_add_item(HEALING_POTION);
        belt.auto_add_item(GREATER_HEALING_POTION);

        belt.auto_add_item(SUPER_HEALING_POTION);

        belt.auto_add_item(MINOR_MANA_POTION);
        belt.auto_add_item(LIGHT_MANA_POTION);
        belt.auto_add_item(MANA_POTION);
        belt.auto_add_item(GREATER_MANA_POTION);

        belt.auto_add_item(SUPER_MANA_POTION);

        belt.auto_add_item(REJUVENATION_POTION);
        belt.auto_add_item(FULL_REJUVENATION_POTION);
        belt.auto_add_item(STAMINA_POTION);

        let expected_belt = vec![
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Greater)),
                None,
                Some(BeltItemType::ManaPotion(ManaPotionType::Greater)),
                None,
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Standard)),
                None,
                Some(BeltItemType::ManaPotion(ManaPotionType::Standard)),
                None,
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Light)),
                None,
                Some(BeltItemType::ManaPotion(ManaPotionType::Light)),
                None,
            ],
            vec![
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor)),
                Some(BeltItemType::HealingPotion(HealingPotionType::Super)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Minor)),
                Some(BeltItemType::ManaPotion(ManaPotionType::Super)),
            ],
        ];

        assert_eq!(belt.items, expected_belt);
    }

    fn test_update_belt_full_belt(
        belt: &mut Belt,
        image_path: &Path,
        consumable_item_matcher: &ConsumableItemsTableMatcher,
        palette_transformer: &PaletteTransformer,
        expected_items: Vec<Vec<Option<BeltItemType>>>,
    ) {
        let img = Image::load_image(image_path);
        let matrix = img.to_matrix(&palette_transformer);

        belt.update_belt(&matrix, &consumable_item_matcher);

        assert_eq!(belt.items, expected_items);
    }

    fn test_belt_consume_item(belt: &mut Belt) {
        let num_rows = belt.table_meta_data.table_size.row as usize;
        let num_cols = belt.table_meta_data.table_size.col as usize;

        belt.clear();

        if num_cols > 0 && num_rows > 2 {
            belt.items[num_rows - 4][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor));
            belt.items[num_rows - 3][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Light));
            belt.items[num_rows - 2][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Greater));
            belt.items[num_rows - 1][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Super));

            belt.consume_item(0);

            assert_eq!(belt.items[num_rows - 4][0], None);
            assert_eq!(
                belt.items[num_rows - 3][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor))
            );
            assert_eq!(
                belt.items[num_rows - 2][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Light))
            );
            assert_eq!(
                belt.items[num_rows - 1][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Greater))
            );

            belt.consume_item(0);

            assert_eq!(belt.items[num_rows - 4][0], None);
            assert_eq!(belt.items[num_rows - 3][0], None);
            assert_eq!(
                belt.items[num_rows - 2][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor))
            );
            assert_eq!(
                belt.items[num_rows - 1][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Light))
            );

            belt.consume_item(0);

            assert_eq!(belt.items[num_rows - 4][0], None);
            assert_eq!(belt.items[num_rows - 3][0], None);
            assert_eq!(belt.items[num_rows - 2][0], None);
            assert_eq!(
                belt.items[num_rows - 1][0],
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor))
            );

            belt.consume_item(0);

            assert_eq!(belt.items[num_rows - 4][0], None);
            assert_eq!(belt.items[num_rows - 3][0], None);
            assert_eq!(belt.items[num_rows - 2][0], None);
            assert_eq!(belt.items[num_rows - 1][0], None);

            belt.consume_item(0);

            assert_eq!(belt.items[num_rows - 4][0], None);
            assert_eq!(belt.items[num_rows - 3][0], None);
            assert_eq!(belt.items[num_rows - 2][0], None);
            assert_eq!(belt.items[num_rows - 1][0], None);
        }
    }

    fn test_belt_is_valid(belt: &mut Belt) {
        let num_rows = belt.table_meta_data.table_size.row as usize;
        let num_cols = belt.table_meta_data.table_size.col as usize;

        belt.clear();
        assert_eq!(belt.is_belt_valid(), true);

        if num_cols > 0 && num_rows > 2 {
            belt.clear();
            belt.items[num_rows - 2][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Light));
            belt.items[num_rows - 1][0] =
                Some(BeltItemType::HealingPotion(HealingPotionType::Minor));

            belt.items[num_rows - 3][1] = Some(BeltItemType::ManaPotion(ManaPotionType::Light));
            belt.items[num_rows - 2][1] = Some(BeltItemType::ManaPotion(ManaPotionType::Light));
            belt.items[num_rows - 1][1] = Some(BeltItemType::ManaPotion(ManaPotionType::Minor));

            belt.items[num_rows - 4][2] = Some(BeltItemType::AntidotePotion);
            belt.items[num_rows - 3][2] = Some(BeltItemType::ThawingPotion);
            belt.items[num_rows - 2][2] = Some(BeltItemType::StaminaPotion);
            belt.items[num_rows - 1][2] = Some(BeltItemType::ScrollOfTownPortal);
            assert_eq!(belt.is_belt_valid(), true);

            belt.clear();
            belt.items[0][0] = Some(BeltItemType::HealingPotion(HealingPotionType::Minor));
            belt.items[1][1] = Some(BeltItemType::HealingPotion(HealingPotionType::Light));
            assert_eq!(belt.is_belt_valid(), false);
        }
    }
}
