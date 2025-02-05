use crate::constants::zone_names::{
    ABADDON, ARREAT_SUMMIT, CRYSTALLINE_PASSAGE, DRIFTER_CAVERN, FRIGID_HIGHLANDS, FROZEN_RIVER,
    FROZEN_TUNDRA, GLACIAL_TRAIL, HALLS_OF_PAIN, ICY_CELLAR, INFERNAL_PIT, PIT_OF_ACHERON,
    THE_ANCIENTS_WAY, THE_CHAOS_SANCTUARY, WORLDSTONE_KEEP_LEVEL_1, WORLDSTONE_KEEP_LEVEL_2,
    WORLDSTONE_KEEP_LEVEL_3,
};

#[derive(Clone, Debug)]
pub struct LevelName(pub String);

impl From<&str> for LevelName {
    fn from(value: &str) -> Self {
        LevelName(english_level_name_to_default_level_name(value))
    }
}

fn english_level_name_to_default_level_name(english_level_name: &str) -> String {
    match english_level_name {
        THE_CHAOS_SANCTUARY => "Chaos Sanctum",

        FRIGID_HIGHLANDS => "Rigid Highlands",
        CRYSTALLINE_PASSAGE => "Crystalized Cavern Level 1",
        GLACIAL_TRAIL => "Crystalized Cavern Level 2",
        FROZEN_RIVER => "Cellar of Pity",
        DRIFTER_CAVERN => "Echo Chamber",
        FROZEN_TUNDRA => "Tundra Wastelands",
        THE_ANCIENTS_WAY => "Glacial Caves Level 1",
        ICY_CELLAR => "Glacial Caves Level 2",
        ARREAT_SUMMIT => "Rocky Summit",

        HALLS_OF_PAIN => "Halls of Death's Calling",

        WORLDSTONE_KEEP_LEVEL_1 => "The Worldstone Keep Level 1",
        WORLDSTONE_KEEP_LEVEL_2 => "The Worldstone Keep Level 2",
        WORLDSTONE_KEEP_LEVEL_3 => "The Worldstone Keep Level 3",

        ABADDON => "Hell1",
        PIT_OF_ACHERON => "Hell2",
        INFERNAL_PIT => "Hell3",
        _ => english_level_name,
    }
    .to_string()
}
