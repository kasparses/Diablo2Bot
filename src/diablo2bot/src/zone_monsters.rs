use crate::{
    constants::misc::{COMPOSITS_TO_SKIP, MODES_TO_SKIP},
    enums::{composit::Composit, game_difficulty::GameDifficulty, mode::Mode},
    level_name::LevelName,
    mpq_archives::{
        excel_levels::ExcelLevels, excel_monstats::ExcelMonstats, excel_monstats2::ExcelMonstats2,
    },
};

#[derive(Debug)]
pub struct CompositEquipments {
    pub composit: Composit,
    pub equipments: Vec<String>,
}

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub code: String,
    pub palshift_id: u8,
    pub base_weapon: String,
    pub composit_equipments: Vec<CompositEquipments>,
    pub modes: Vec<Mode>,
}

pub struct DccFile {
    pub name: String,
    pub full_path: String,
}

impl Monster {
    pub fn get_dcc_file_paths(&self) -> Vec<DccFile> {
        let mut paths = Vec::new();

        for composit_eqipment in &self.composit_equipments {
            for equipment in &composit_eqipment.equipments {
                for mode in &self.modes {
                    let name = format!(
                        "{}{}{}{}{}.dcc",
                        self.code, composit_eqipment.composit, equipment, mode, self.base_weapon
                    )
                    .to_string();

                    let path = format!(
                        "data\\global\\monsters\\{}\\{}\\{}",
                        self.code, composit_eqipment.composit, name
                    );

                    paths.push(DccFile {
                        name,
                        full_path: path,
                    });
                }
            }
        }

        paths
    }
}

pub fn get_monster(
    monstats: &ExcelMonstats,
    monstats2: &ExcelMonstats2,
    monster_id: &str,
) -> Monster {
    let monstats_row = monstats.get_row(monster_id);
    let monstats2_row = monstats2.get_row(monstats_row.monstats2_id);

    let base_weapon = monstats2_row.base_weapon.to_string();

    let palshift_id = monstats_row.palshift_id as u8;
    let name = monstats_row.name.to_string();
    let code = monstats_row.code.to_string();

    let composits = monstats2_row
        .get_composit_equipments()
        .into_iter()
        .filter(|c| !COMPOSITS_TO_SKIP.contains(&c.composit))
        .collect();

    let modes = monstats2_row
        .get_modes()
        .into_iter()
        .filter(|mode| !MODES_TO_SKIP.contains(mode))
        .collect();

    Monster {
        palshift_id,
        name,
        code,
        composit_equipments: composits,
        modes,
        base_weapon,
    }
}

pub fn get_monsters_in_level(
    level: LevelName,
    levels: &ExcelLevels,
    monstats: &ExcelMonstats,
    monstats2: &ExcelMonstats2,
    game_difficulty: GameDifficulty,
) -> Vec<Monster> {
    levels
        .get_monster_ids(level, game_difficulty, monstats)
        .iter()
        .map(|id| get_monster(monstats, monstats2, id))
        .collect()
}
