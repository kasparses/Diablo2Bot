use std::collections::HashMap;

use crate::{
    enums::character_class::CharacterClass,
    matrix::Matrix,
    mpq_archives::{archives::Archives, excel_skills::SkillData},
    point_u16::PointU16,
};

pub struct SkillIconGetter {
    skill_name_to_icon_sprite: HashMap<String, Matrix>,
}

impl SkillIconGetter {
    pub fn new(archives: &mut Archives, skill_names: &[&str]) -> Self {
        let excel_skills_raw_text = archives.extract_excel_skills_raw_text().unwrap();
        let excel_skills = excel_skills_raw_text.parse();

        let skill_class_to_skills = excel_skills.group_skills_by_class(skill_names);

        let skill_name_to_icon_sprite =
            Self::create_skill_name_to_icon_sprite_map(skill_class_to_skills, archives);

        Self {
            skill_name_to_icon_sprite,
        }
    }

    pub fn get_skill_icon_sprite(&self, skill_name: &str) -> &Matrix {
        self.skill_name_to_icon_sprite.get(skill_name).unwrap()
    }

    fn create_skill_name_to_icon_sprite_map(
        skill_class_to_skills: HashMap<Option<CharacterClass>, Vec<SkillData>>,
        archives: &mut Archives,
    ) -> HashMap<String, Matrix> {
        let mut skill_name_to_icon_sprite = HashMap::new();

        for (skill_class, skill_data) in skill_class_to_skills {
            let skill_icon_dc6_bytes = match skill_class {
                Some(class) => archives.extract_class_skill_icon_dc6_bytes(class),
                None => archives.extract_general_skill_icon_dc6_bytes(),
            }
            .unwrap();

            let skill_icon_dc6_file = skill_icon_dc6_bytes.parse();

            for skill in skill_data {
                let mut matrix = Matrix::from_dc6_encoded_frame(
                    &skill_icon_dc6_file.directions[0].encoded_frames
                        [skill.icon_sprite_id as usize],
                );

                matrix.remove_last_row();
                matrix.set_value(PointU16::new(0, 0), 0);

                skill_name_to_icon_sprite.insert(skill.name.to_string(), matrix);
            }
        }
        skill_name_to_icon_sprite
    }
}
