use serde::Deserialize;

use crate::units::{Frames, Milliseconds};

#[derive(Deserialize, Debug, Clone)]
pub struct BotSettings {
    pub save_logs: bool,
    pub max_game_runs: u32,
    pub pre_cache_connected_areas: bool,
    pub pre_cache_connected_areas_multiple_threads: bool,
    pub enable_mouse_movement_program_stopper: bool,
    pub match_unique_and_champion_monsters: bool,
    pub movement_settings: MovementSettings,
    pub loot_settings: LootSettings,
    pub max_frames_to_wait_for_ui_action: u32,
    pub max_frames_to_wait_for_enter_game: u32,
    pub max_frames_to_wait_for_exit_game: u32,
    pub max_frames_to_wait_for_zone_load: u32,
    pub max_frames_to_wait_for_locate_game_window: u32,
    pub max_windows_per_sprite_frame: u32,
    pub num_frames_to_sleep_after_lifting_held_key: Frames,
    pub num_frames_to_sleep_before_looting_after_attacking_monsters: Frames,
    pub num_frames_to_sleep_after_attacking_monsters: Frames,
    pub num_frames_to_sleep_after_scanning_screen_for_monsters: Frames,
    pub num_frames_to_sleep_after_casting_buffs_on_secondary_weaponset: Frames,
    pub stash_settings: StashSettings,
    pub merchant_purchase_cooldown_frames: Frames,
    pub game_startup_settings: GameStartupSettings,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct StashSettings {
    pub num_frames_to_sleep_after_picking_up_item_from_inventory_before_moving_it_to_stash: Frames,
    pub num_frames_to_sleep_after_placing_item_in_stash: Frames,
    pub num_frames_to_sleep_after_placing_items_in_stash: Frames,
    pub num_frames_to_sleep_after_moving_gold_to_stash: Frames,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct LootSettings {
    pub max_potions_to_pickup_per_loot_session: u32,
    pub max_items_to_pickup_per_loot_session: u32,
    pub max_gold_piles_to_pickup_per_loot_session: u32,
    pub num_frames_to_sleep_after_activating_loot_text: Frames,
    pub num_frames_to_sleep_after_picking_up_item: Frames,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct MovementSettings {
    pub max_movements_before_automap_path_refresh: u32,
    pub max_automap_path_refresh_before_game_refresh: u32,
    pub num_random_destination_points_to_choose_from: u32,
    pub max_num_tiles_from_path_to_mark_as_walked: u32,
    pub wide_start_size: u32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct GameStartupSettings {
    pub check_game_started_cooldown_milliseconds: Milliseconds,
    pub max_milliseconds_check_game_started: u64,
}
