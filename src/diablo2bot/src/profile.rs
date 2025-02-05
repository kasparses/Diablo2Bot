use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::enums::{
    character_class::CharacterClass, game_difficulty::GameDifficulty,
    game_interface_element::GameInterfaceElement, operating_system::OperatingSystem,
    waypoint_zone::WaypointZone,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub character_name: String,
    pub item_filter: String,
    pub character_class: CharacterClass,
    pub primary_attack_skill: String,
    pub faster_cast_rate_weaponset_primary: u32,
    pub faster_cast_rate_weaponset_secondary: u32,
    pub left_skill_weaponset_primary: String,
    pub left_skill_weaponset_secondary: String,
    #[serde(deserialize_with = "validate_zone_to_farm")]
    pub zone_to_farm: WaypointZone,
    pub min_gold_to_pickup: u32,
    pub game_difficulty: GameDifficulty,
    pub num_belt_columns_reserved_for_healing_potions: u8,
    pub num_belt_columns_reserved_for_mana_potions: u8,
    pub health_limit: f32,
    pub health_limit_hard: f32,
    pub mana_limit: f32,
    #[serde(deserialize_with = "validate_players_count")]
    pub players_count: u8,
    pub set_game_options: bool,
    pub keybindings: KeyBindings,
    pub buffs: Vec<Buff>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemSettings {
    pub diablo2_folder_path: String,
    pub operating_system: OperatingSystem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyBindings {
    pub skills: HashMap<String, String>, // skill name to keybinding
    pub game_interface_actions: HashMap<GameInterfaceElement, String>,
    pub miscellaneous: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buff {
    pub skill: String,
    pub duration: u32,
    pub only_castable_on_secondary_weaponset: bool,
}

fn validate_zone_to_farm<'de, D>(deserializer: D) -> Result<WaypointZone, D::Error>
where
    D: Deserializer<'de>,
{
    let zone: WaypointZone = Deserialize::deserialize(deserializer)?;

    match zone {
        WaypointZone::RogueEncampment
        | WaypointZone::LutGholein
        | WaypointZone::KurastDocks
        | WaypointZone::ThePandemoniumFortress
        | WaypointZone::Harrogath => Err(serde::de::Error::custom(
            "Your chosen zone to farm is a town zone which contains no monsters!",
        )),
        _ => Ok(zone),
    }
}

fn validate_players_count<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let number: u8 = Deserialize::deserialize(deserializer)?;
    if number <= 8 {
        Ok(number)
    } else {
        Err(serde::de::Error::custom(
            "Player count must not be greater than 8",
        ))
    }
}
