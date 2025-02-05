use super::character_class::CharacterClass;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ManaPotionType {
    Minor,
    Light,
    Standard,
    Greater,
    Super,
}

impl ManaPotionType {
    pub fn get_points(self, class: CharacterClass) -> u32 {
        match class {
            CharacterClass::Druid | CharacterClass::Necromancer | CharacterClass::Sorceress => {
                match self {
                    Self::Minor => 40,
                    Self::Light => 80,
                    Self::Standard => 160,
                    Self::Greater => 300,
                    Self::Super => 500,
                }
            }
            CharacterClass::Amazon | CharacterClass::Paladin | CharacterClass::Assassin => {
                match self {
                    Self::Minor => 30,
                    Self::Light => 60,
                    Self::Standard => 120,
                    Self::Greater => 225,
                    Self::Super => 375,
                }
            }
            CharacterClass::Barbarian => match self {
                Self::Minor => 20,
                Self::Light => 40,
                Self::Standard => 80,
                Self::Greater => 150,
                Self::Super => 250,
            },
        }
    }
}
