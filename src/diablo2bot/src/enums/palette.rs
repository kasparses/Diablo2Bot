use std::fmt;

use super::act::Act;

#[allow(dead_code)]
pub enum Palette {
    Act(Act),
    EndGame,
    FeChar,
    Loading,
    Menu0,
    Menu1,
    Menu2,
    Menu3,
    Menu4,
    Sky,
    Trademark,
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Palette::Act(a) => format!("{}", a),
            Palette::EndGame => "EndGame".to_string(),
            Palette::FeChar => "FeChar".to_string(),
            Palette::Loading => "Loading".to_string(),
            Palette::Menu0 => "Menu0".to_string(),
            Palette::Menu1 => "Menu1".to_string(),
            Palette::Menu2 => "Menu2".to_string(),
            Palette::Menu3 => "Menu3".to_string(),
            Palette::Menu4 => "Menu4".to_string(),
            Palette::Sky => "Sky".to_string(),
            Palette::Trademark => "Trademark".to_string(),
        };

        write!(f, "{s}")
    }
}

impl From<Act> for Palette {
    fn from(act: Act) -> Self {
        Self::Act(act)
    }
}
