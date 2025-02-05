use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constants::zone_names::{
    ARCANE_SANCTUARY, ARREAT_PLATEAU, BLACK_MARSH, CANYON_OF_THE_MAGI, CATACOMBS_LEVEL_2,
    CITY_OF_THE_DAMNED, COLD_PLAINS, CRYSTALLINE_PASSAGE, DARK_WOOD, DRY_HILLS,
    DURANCE_OF_HATE_LEVEL_2, FAR_OASIS, FLAYER_JUNGLE, FRIGID_HIGHLANDS, FROZEN_TUNDRA,
    GLACIAL_TRAIL, GREAT_MARSH, HALLS_OF_PAIN, HALLS_OF_THE_DEAD_LEVEL_2, HARROGATH,
    INNER_CLOISTER, JAIL_LEVEL_1, KURAST_BAZAAR, KURAST_DOCKS, LOST_CITY, LOWER_KURAST,
    LUT_GHOLEIN, OUTER_CLOISTER, PALACE_CELLAR_LEVEL_1, RIVER_OF_FLAME, ROGUE_ENCAMPMENT,
    SEWERS_LEVEL_2, SPIDER_FOREST, STONY_FIELD, THE_ANCIENTS_WAY, THE_PANDEMONIUM_FORTRESS,
    TRAVINCAL, UPPER_KURAST, WORLDSTONE_KEEP_LEVEL_2,
};

use super::act::Act;

#[derive(Serialize, Deserialize, Debug)]
pub enum WaypointZone {
    #[serde(rename = "Rogue Encampment")]
    RogueEncampment,
    #[serde(rename = "Lut Gholein")]
    LutGholein,
    #[serde(rename = "Kurast Docks")]
    KurastDocks,
    #[serde(rename = "The Pandemonium Fortress")]
    ThePandemoniumFortress,
    #[serde(rename = "Harrogath")]
    Harrogath,
    #[serde(rename = "Cold Plains")]
    ColdPlains,
    #[serde(rename = "Sewers Level 2")]
    SewersLevel2,
    #[serde(rename = "Spider Forest")]
    SpiderForest,
    #[serde(rename = "City of the Damned")]
    CityOfTheDamned,
    #[serde(rename = "Frigid Highlands")]
    FrigidHighlands,
    #[serde(rename = "Stony Field")]
    StonyField,
    #[serde(rename = "Dry Hills")]
    DryHills,
    #[serde(rename = "Great Marsh")]
    GreatMarsh,
    #[serde(rename = "River of Flame")]
    RiverOfFlame,
    #[serde(rename = "Arreat Plateau")]
    ArreatPlateau,
    #[serde(rename = "Dark Wood")]
    DarkWood,
    #[serde(rename = "Halls of the Dead Level 2")]
    HallsOfTheDeadLevel2,
    #[serde(rename = "Flayer Jungle")]
    FlayerJungle,
    #[serde(rename = "Crystalline Passage")]
    CrystallinePassage,
    #[serde(rename = "Black Marsh")]
    BlackMarsh,
    #[serde(rename = "Far Oasis")]
    FarOasis,
    #[serde(rename = "Lower Kurast")]
    LowerKurast,
    #[serde(rename = "Glacial Trail")]
    GlacialTrail,
    #[serde(rename = "Outer Cloister")]
    OuterCloister,
    #[serde(rename = "Lost City")]
    LostCity,
    #[serde(rename = "Kurast Bazaar")]
    KurastBazaar,
    #[serde(rename = "Halls of Pain")]
    HallsOfPain,
    #[serde(rename = "Jail Level 1")]
    JailLevel1,
    #[serde(rename = "Palace Cellar Level 1")]
    PalaceCellarLevel1,
    #[serde(rename = "Upper Kurast")]
    UpperKurast,
    #[serde(rename = "Frozen Tundra")]
    FrozenTundra,
    #[serde(rename = "Inner Cloister")]
    InnerCloister,
    #[serde(rename = "Arcane Sanctuary")]
    ArcaneSanctuary,
    #[serde(rename = "Travincal")]
    Travincal,
    #[serde(rename = "The Ancients' Way")]
    TheAncientsWay,
    #[serde(rename = "Catacombs Level 2")]
    CatacombsLevel2,
    #[serde(rename = "Canyon of the Magi")]
    CanyonOfTheMagi,
    #[serde(rename = "Durance of Hate Level 2")]
    DuranceOfHateLevel2,
    #[serde(rename = "Worldstone Keep Level 2")]
    WorldstoneKeepLevel2,
}

impl WaypointZone {
    pub fn to_act(&self) -> Act {
        match self {
            Self::RogueEncampment
            | Self::ColdPlains
            | Self::StonyField
            | Self::DarkWood
            | Self::BlackMarsh
            | Self::OuterCloister
            | Self::JailLevel1
            | Self::InnerCloister
            | Self::CatacombsLevel2 => Act::Act1,

            Self::LutGholein
            | Self::SewersLevel2
            | Self::DryHills
            | Self::HallsOfTheDeadLevel2
            | Self::FarOasis
            | Self::LostCity
            | Self::PalaceCellarLevel1
            | Self::ArcaneSanctuary
            | Self::CanyonOfTheMagi => Act::Act2,

            Self::KurastDocks
            | Self::SpiderForest
            | Self::GreatMarsh
            | Self::FlayerJungle
            | Self::LowerKurast
            | Self::KurastBazaar
            | Self::UpperKurast
            | Self::Travincal
            | Self::DuranceOfHateLevel2 => Act::Act3,

            Self::ThePandemoniumFortress | Self::CityOfTheDamned | Self::RiverOfFlame => Act::Act4,

            Self::Harrogath
            | Self::FrigidHighlands
            | Self::ArreatPlateau
            | Self::CrystallinePassage
            | Self::GlacialTrail
            | Self::HallsOfPain
            | Self::FrozenTundra
            | Self::TheAncientsWay
            | Self::WorldstoneKeepLevel2 => Act::Act5,
        }
    }
}

impl fmt::Display for WaypointZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::RogueEncampment => ROGUE_ENCAMPMENT,
            Self::ColdPlains => COLD_PLAINS,
            Self::StonyField => STONY_FIELD,
            Self::DarkWood => DARK_WOOD,
            Self::BlackMarsh => BLACK_MARSH,
            Self::OuterCloister => OUTER_CLOISTER,
            Self::JailLevel1 => JAIL_LEVEL_1,
            Self::InnerCloister => INNER_CLOISTER,
            Self::CatacombsLevel2 => CATACOMBS_LEVEL_2,

            Self::LutGholein => LUT_GHOLEIN,
            Self::SewersLevel2 => SEWERS_LEVEL_2,
            Self::DryHills => DRY_HILLS,
            Self::HallsOfTheDeadLevel2 => HALLS_OF_THE_DEAD_LEVEL_2,
            Self::FarOasis => FAR_OASIS,
            Self::LostCity => LOST_CITY,
            Self::PalaceCellarLevel1 => PALACE_CELLAR_LEVEL_1,
            Self::ArcaneSanctuary => ARCANE_SANCTUARY,
            Self::CanyonOfTheMagi => CANYON_OF_THE_MAGI,

            Self::KurastDocks => KURAST_DOCKS,
            Self::SpiderForest => SPIDER_FOREST,
            Self::GreatMarsh => GREAT_MARSH,
            Self::FlayerJungle => FLAYER_JUNGLE,
            Self::LowerKurast => LOWER_KURAST,
            Self::KurastBazaar => KURAST_BAZAAR,
            Self::UpperKurast => UPPER_KURAST,
            Self::Travincal => TRAVINCAL,
            Self::DuranceOfHateLevel2 => DURANCE_OF_HATE_LEVEL_2,

            Self::ThePandemoniumFortress => THE_PANDEMONIUM_FORTRESS,
            Self::CityOfTheDamned => CITY_OF_THE_DAMNED,
            Self::RiverOfFlame => RIVER_OF_FLAME,

            Self::Harrogath => HARROGATH,
            Self::FrigidHighlands => FRIGID_HIGHLANDS,
            Self::ArreatPlateau => ARREAT_PLATEAU,
            Self::CrystallinePassage => CRYSTALLINE_PASSAGE,
            Self::GlacialTrail => GLACIAL_TRAIL,
            Self::HallsOfPain => HALLS_OF_PAIN,
            Self::FrozenTundra => FROZEN_TUNDRA,
            Self::TheAncientsWay => THE_ANCIENTS_WAY,
            Self::WorldstoneKeepLevel2 => WORLDSTONE_KEEP_LEVEL_2,
        };

        write!(f, "{s}")
    }
}
