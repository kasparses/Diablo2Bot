use std::collections::HashMap;

use crate::{
    constants::zone_names::{STONY_FIELD, TAMOE_HIGHLAND},
    enums::game_difficulty::GameDifficulty,
    level_name::LevelName,
};

use super::excel_monstats::ExcelMonstats;

pub struct ExcelLevelsRawText {
    text: String,
}

impl ExcelLevelsRawText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ExcelLevels {
        ExcelLevels::new(&self.text)
    }
}

struct Row<'raw_text> {
    level_name: &'raw_text str,
    monster_names_normal: [Option<&'raw_text str>; 10],
    monster_names_nightmare_hell: [Option<&'raw_text str>; 10],
}

pub struct ExcelLevels<'raw_text> {
    rows: Vec<Row<'raw_text>>,
}

impl<'raw_text> ExcelLevels<'raw_text> {
    pub fn new(text: &'raw_text str) -> Self {
        let separator = '\t';

        let mut line_iter = text.split("\r\n");

        let mut column_headers = HashMap::new();
        let mut num_columns = 0;

        if let Some(header_line) = line_iter.next() {
            for (i, header) in header_line.split(separator).enumerate() {
                num_columns += 1;
                column_headers.insert(header, i);
            }
        }

        let area_name_col_id = column_headers["Name"];
        let level_name_col_id = column_headers["LevelName"];
        let mon1 = column_headers["mon1"];
        let nmon1 = column_headers["nmon1"];

        let mut row: Vec<&str> = vec![""; num_columns];

        let mut parsed_rows = Vec::new();

        for line in line_iter {
            for (i, column) in line.split(separator).enumerate() {
                row[i] = column;
            }

            let area_name = row[area_name_col_id];

            if area_name == "Expansion" || area_name == "Null" {
                continue;
            }

            let level_name = row[level_name_col_id];

            let mut monster_names_normal = [None; 10];
            let mut monster_names_nightmare_hell = [None; 10];

            for i in 0..10 {
                let monster_name = row[mon1 + i];
                if !monster_name.is_empty() {
                    monster_names_normal[i] = Some(monster_name);
                }

                let monster_name = row[nmon1 + i];
                if !monster_name.is_empty() {
                    monster_names_nightmare_hell[i] = Some(monster_name);
                }
            }

            parsed_rows.push(Row {
                level_name,
                monster_names_normal,
                monster_names_nightmare_hell,
            })
        }

        Self { rows: parsed_rows }
    }

    fn get_extra_monster_ids(level_name: &str) -> Vec<&'raw_text str> {
        match level_name {
            STONY_FIELD => vec!["fallenshaman1", "fallenshaman2"], // Spwaned in the carver/fallen camp area.
            TAMOE_HIGHLAND => vec!["fallenshaman2", "fallenshaman3"], // Spwaned in the carver/fallen camp area.
            _ => Vec::new(),
        }
    }

    fn get_spawn_monster_ids(
        monstats: &'raw_text ExcelMonstats,
        monster_ids: &[&str],
    ) -> Vec<&'raw_text str> {
        monster_ids
            .iter()
            .filter_map(|monster_id| monstats.get_row(monster_id).spawn_id)
            .collect()
    }

    fn get_minion_monster_ids(
        monstats: &'raw_text ExcelMonstats,
        monster_ids: &[&str],
    ) -> Vec<&'raw_text str> {
        monster_ids
            .iter()
            .map(|&monster_id| monstats.get_row(monster_id))
            .flat_map(|monstats_row| monstats_row.minion_ids.iter().flatten())
            .cloned()
            .collect()
    }

    pub fn get_monster_ids(
        &self,
        level_name: LevelName,
        game_difficulty: GameDifficulty,
        monstats: &'raw_text ExcelMonstats,
    ) -> Vec<&'raw_text str> {
        for row in &self.rows {
            if row.level_name == level_name.0 {
                let monster_names = match game_difficulty {
                    GameDifficulty::Normal => row.monster_names_normal,
                    _ => row.monster_names_nightmare_hell,
                };

                let mut monster_ids: Vec<_> = monster_names.into_iter().flatten().collect();
                monster_ids.extend(Self::get_extra_monster_ids(&level_name.0));

                let spawn_monster_ids = Self::get_spawn_monster_ids(monstats, &monster_ids);
                let minion_monster_ids = Self::get_minion_monster_ids(monstats, &monster_ids);

                monster_ids.extend(spawn_monster_ids);
                monster_ids.extend(minion_monster_ids);

                monster_ids.sort();
                monster_ids.dedup();

                return monster_ids;
            }
        }

        panic!("Could not find level {}", level_name.0);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{file_io::FileIo, mpq_archives::archives::Archives};

    #[test]
    fn test_excel_levels() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let excel_levels_raw_text = archives.extract_excel_levels_raw_text().unwrap();
        let now = Instant::now();
        let excel_levels = excel_levels_raw_text.parse();
        println!("elapsed: {:?} micros", now.elapsed().as_micros());
        println!("{}", excel_levels.rows.len());
    }
}
