use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    Necromancer,
    Sorceress,
    Druid,
    Amazon,
    Paladin,
    Assassin,
    Barbarian,
}

impl CharacterClass {
    pub fn get_class_code(&self) -> &'static str {
        match self {
            Self::Necromancer => "Ne",
            Self::Sorceress => "So",
            Self::Druid => "Dr",
            Self::Amazon => "Am",
            Self::Paladin => "Pa",
            Self::Assassin => "As",
            Self::Barbarian => "Ba",
        }
    }
}
