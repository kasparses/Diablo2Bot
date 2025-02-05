use super::character_class::CharacterClass;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HealingPotionType {
    Minor,
    Light,
    Standard,
    Greater,
    Super,
}

impl HealingPotionType {
    pub fn get_points(self, class: CharacterClass) -> u32 {
        match class {
            CharacterClass::Druid | CharacterClass::Necromancer | CharacterClass::Sorceress => {
                match self {
                    Self::Minor => 30,
                    Self::Light => 60,
                    Self::Standard => 100,
                    Self::Greater => 180,
                    Self::Super => 320,
                }
            }
            CharacterClass::Amazon | CharacterClass::Paladin | CharacterClass::Assassin => {
                match self {
                    Self::Minor => 45,
                    Self::Light => 90,
                    Self::Standard => 150,
                    Self::Greater => 270,
                    Self::Super => 480,
                }
            }
            CharacterClass::Barbarian => match self {
                Self::Minor => 60,
                Self::Light => 120,
                Self::Standard => 200,
                Self::Greater => 360,
                Self::Super => 640,
            },
        }
    }
}
