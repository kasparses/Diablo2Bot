use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    constants::game_window_points::{
        WAYPOINT_ACT1_TAB, WAYPOINT_ACT2_TAB, WAYPOINT_ACT3_TAB, WAYPOINT_ACT4_TAB,
        WAYPOINT_ACT5_TAB,
    },
    point_u16::PointU16,
};

use super::route::Route;

pub const ACTS: [Act; 5] = [Act::Act1, Act::Act2, Act::Act3, Act::Act4, Act::Act5];

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Act {
    #[serde(rename = "ACT1")]
    Act1,
    #[serde(rename = "ACT2")]
    Act2,
    #[serde(rename = "ACT3")]
    Act3,
    #[serde(rename = "ACT4")]
    Act4,
    #[serde(rename = "ACT5")]
    Act5,
}

impl Act {
    pub fn get_potion_seller_name(self) -> String {
        match self {
            Self::Act1 => "Akara",
            Self::Act2 => "Lysander",
            Self::Act3 => "Ormus",
            Self::Act4 => "Jamella",
            Self::Act5 => "Malah",
        }
        .to_string()
    }

    pub fn get_deckard_cain_monster_id(self) -> String {
        match self {
            Self::Act1 => "cain1",
            Self::Act2 => "cain2",
            Self::Act3 => "cain3",
            Self::Act4 => "cain4",
            Self::Act5 => "cain5",
        }
        .to_string()
    }

    pub fn to_waypoint_act_tab(self) -> PointU16 {
        match self {
            Self::Act1 => WAYPOINT_ACT1_TAB,
            Self::Act2 => WAYPOINT_ACT2_TAB,
            Self::Act3 => WAYPOINT_ACT3_TAB,
            Self::Act4 => WAYPOINT_ACT4_TAB,
            Self::Act5 => WAYPOINT_ACT5_TAB,
        }
    }

    pub fn get_deckard_cain_route(self: Act) -> Route {
        match self {
            Self::Act1 => Route::Act1StartToDeckardCain,
            Self::Act2 => Route::Act2StartToDeckardCain,
            Self::Act3 => Route::Act3StartToDeckardCain,
            Self::Act4 => Route::Act4StartToDeckardCain,
            Self::Act5 => Route::Act5StartToDeckardCain,
        }
    }

    pub fn get_potion_seller_route(self: Act) -> Route {
        match self {
            Self::Act1 => Route::Act1StartToPotionSeller,
            Self::Act2 => Route::Act2StartToPotionSeller,
            Self::Act3 => Route::Act3StartToPotionSeller,
            Self::Act4 => Route::Act4StartToPotionSeller,
            Self::Act5 => Route::Act5StartToPotionSeller,
        }
    }

    pub fn get_stash_route(self: Act) -> Route {
        match self {
            Self::Act1 => Route::Act1StartToStash,
            Self::Act2 => Route::Act2StartToStash,
            Self::Act3 => Route::Act3StartToStash,
            Self::Act4 => Route::Act4StartToStash,
            Self::Act5 => Route::Act5StartToStash,
        }
    }
}

impl fmt::Display for Act {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Act1 => "ACT1",
            Self::Act2 => "ACT2",
            Self::Act3 => "ACT3",
            Self::Act4 => "ACT4",
            Self::Act5 => "ACT5",
        };

        write!(f, "{s}")
    }
}
