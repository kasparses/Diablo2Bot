use crate::enums::act::Act;

use super::zone_names::{
    HARROGATH, KURAST_DOCKS, LUT_GHOLEIN, ROGUE_ENCAMPMENT, THE_PANDEMONIUM_FORTRESS,
};

pub const TOWN_LEVELS: [&str; 5] = [
    ROGUE_ENCAMPMENT,
    LUT_GHOLEIN,
    KURAST_DOCKS,
    THE_PANDEMONIUM_FORTRESS,
    HARROGATH,
];

pub fn town_level_to_act(town_level: &str) -> Option<Act> {
    match town_level {
        ROGUE_ENCAMPMENT => Some(Act::Act1),
        LUT_GHOLEIN => Some(Act::Act2),
        KURAST_DOCKS => Some(Act::Act3),
        THE_PANDEMONIUM_FORTRESS => Some(Act::Act4),
        HARROGATH => Some(Act::Act5),
        _ => None,
    }
}
