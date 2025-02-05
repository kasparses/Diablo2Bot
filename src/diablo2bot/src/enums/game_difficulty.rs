use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GameDifficulty {
    Normal,
    Nightmare,
    Hell,
}

impl GameDifficulty {
    pub fn to_keyboard_shortcut_key(self) -> char {
        match self {
            Self::Normal => 'r',
            Self::Nightmare => 'n',
            Self::Hell => 'h',
        }
    }
}
