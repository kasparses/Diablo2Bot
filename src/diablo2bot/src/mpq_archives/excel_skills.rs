use std::collections::HashMap;

use crate::enums::character_class::CharacterClass;

pub struct ExcelSkillsRawText {
    text: String,
}

impl ExcelSkillsRawText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ExcelSkills {
        ExcelSkills::new(&self.text)
    }
}

pub struct SkillData<'skill_names> {
    pub name: &'skill_names str,
    pub icon_sprite_id: u32,
}

struct Row<'raw_text> {
    skill_name: &'raw_text str,
    skill_icon_sprite_id: u32,
    skill_class: Option<CharacterClass>,
}

pub struct ExcelSkills<'raw_text> {
    rows: Vec<Row<'raw_text>>,
}

impl<'raw_text> ExcelSkills<'raw_text> {
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

        let skill_name_col_id = column_headers["skill"];
        let skill_icon_sprite_id_col_id = column_headers["IconCel"];
        let skill_class_col_id = column_headers["charclass"];

        let mut row: Vec<&str> = vec![""; num_columns];

        let mut parsed_rows = Vec::new();

        for line in line_iter {
            for (i, column) in line.split(separator).enumerate() {
                row[i] = column;
            }

            let skill_name = row[skill_name_col_id];

            if skill_name == "Expansion" {
                continue;
            }

            let skill_icon_sprite_id = row[skill_icon_sprite_id_col_id].parse::<u32>().unwrap();

            let skill_class = match row[skill_class_col_id] {
                "ama" => Some(CharacterClass::Amazon),
                "sor" => Some(CharacterClass::Sorceress),
                "nec" => Some(CharacterClass::Necromancer),
                "pal" => Some(CharacterClass::Paladin),
                "bar" => Some(CharacterClass::Barbarian),
                "dru" => Some(CharacterClass::Druid),
                "ass" => Some(CharacterClass::Assassin),
                _ => None,
            };

            parsed_rows.push(Row {
                skill_name,
                skill_icon_sprite_id,
                skill_class,
            })
        }

        Self { rows: parsed_rows }
    }

    pub fn group_skills_by_class<'skill_names>(
        &self,
        skill_names: &'skill_names [&'skill_names str],
    ) -> HashMap<Option<CharacterClass>, Vec<SkillData<'skill_names>>> {
        let mut skill_class_to_skill_data = HashMap::new();

        for skill_name in skill_names {
            let mut found_skill = false;

            for row in &self.rows {
                if row.skill_name == *skill_name {
                    found_skill = true;

                    skill_class_to_skill_data
                        .entry(row.skill_class)
                        .or_insert_with(Vec::new)
                        .push(SkillData {
                            name: skill_name,
                            icon_sprite_id: row.skill_icon_sprite_id,
                        });
                }
            }

            if !found_skill {
                panic!("Could not find skill with name: {skill_name}")
            }
        }

        skill_class_to_skill_data
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{file_io::FileIo, mpq_archives::archives::Archives};

    #[test]
    fn test_excel_skills() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let now = Instant::now();
        let excel_skills_raw_text = archives.extract_excel_skills_raw_text().unwrap();
        let excel_skills = excel_skills_raw_text.parse();
        println!("elapsed: {:?} micros", now.elapsed().as_micros());
        println!("{}", excel_skills.rows.len());
    }
}
