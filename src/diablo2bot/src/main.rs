use bot_settings::BotSettings;
use buy_potions::buy_potions;
use constants::game_window_areas::{WAYPOINT_TEXT_AREA, ZONE_NAME_AREA};
use constants::game_window_points::{SAVE_AND_EXIT_GAME_POINT, TOP_LEFT_CHARACTER_POINT};
use constants::misc::GAME_WINDOW_SIZE;
use constants::town_levels::TOWN_LEVELS;
use constants::validation_pixels::{
    EXIT_GAME_VALIDATION_PIXELS, START_SCREEN_LOADING_SCREEN_VALIDATION_PIXELS,
};
use drop_unwanted_items::drop_unwanted_items;
use enigo::{Enigo, MouseControllable};
use enums::act::Act;
use enums::belt_item_type::HealthManaPotionType;
use enums::click_type::ClickType;
use enums::errors::{BotError, MovedToTownZoneError, WrongGameStateError};
use enums::game_difficulty::GameDifficulty;
use enums::game_interface_element::GameInterfaceElement::{Automap, Belt, Portraits};
use enums::quality::Quality;
use enums::route::Route;
use enums::state::State;
use file_io::FileIo;
use game::Game;
use game_interface_element_controller::GameInterfaceElementController;
use game_screenshotter::GameScreenshotter;
use game_window_activator::start_diablo2;
use get_path_and_walk_it::get_path_and_walk_it;
use go_to_act::go_to_act;
use health_mana::get_points_and_drink_potions_if_points_under_soft_limit;
use identify_items::identify_items_at_deckard_cain;
use identify_starting_act::identify_starting_act;
use image::Image;
use loot::get_inventory_table;
use match_text_with_palette::{get_font_char_map, match_unique_text_with_palette};
use matrix::Matrix;
use mouse_movement_program_stopper::setup_mouse_movement_program_stopper;
use mpq_archives::archives::Archives;
use options::{enter_single_player_menu, open_game_menu, set_automap_options, set_video_options};
use output_controller::OutputController;
use pattern_matcher_monsters::MonsterMatcherConfig;
use point_u16::PointU16;
use route_walker::{is_last_element, walk_enum_route_with_end_state};
use run::run;
use screenshotter::Screenshotter;
use state_validator::{
    wait_for_enum_state, wait_for_enum_state_, wait_for_enum_states, wait_while_in_enum_state,
    ValidationPixel,
};
use std::env;
use std::process::exit;
use toogle_health_and_mana_text::ensure_health_and_mana_text_is_toggled_on;
use units::Milliseconds;
use utils::{sleep_frame, sleep_millis};
use zone_to_area::zone_to_area;
use zones::is_valid_zone;

use structs::{Item, Pixel};

use crate::pattern_matcher_monsters::get_monster_tree;

mod attack_monsters;
mod belt;
mod bot_settings;
mod box_u16;
mod buffs;
mod buy_potions;
mod constants;
mod drop_unwanted_items;
mod enums;
mod fast_hash_set;
mod file_io;
mod find_npc;
mod font_char_map;
mod font_matcher;
mod game;
mod game_interface_element_controller;
mod game_screenshotter;
mod game_window_activator;
mod get_path_and_walk_it;
mod go_to_act;
mod health_mana;
mod identify_items;
mod identify_starting_act;
mod image;
mod inventory;
mod level_name;
mod logger;
mod loot;
mod map_matcher;
mod match_text_with_palette;
mod matrix;
mod mouse_movement_program_stopper;
mod move_items_to_stash;
mod mpq_archives;
mod options;
mod output_controller;
mod pal_pl2;
mod palette;
mod pattern_matcher;
mod pattern_matcher2;
mod pattern_matcher_monsters;
mod point_i32;
mod point_u16;
mod point_u8;
mod pre_cache_connected_areas;
mod profile;
mod quality_palette;
mod route_walker;
mod run;
mod screenshotter;
mod skill_icon_getter;
mod spell_caster;
mod state_validator;
mod string_tables;
mod structs;
mod table;
mod table_matcher;
mod test_utils;
mod tile_mask_getter;
mod toogle_health_and_mana_text;
mod units;
mod utils;
mod weapon_swapper;
mod weaponset_data;
mod zone_monsters;
mod zone_to_area;
mod zone_traveller;
mod zones;

fn _set_players_count(players_count: u8, output_controller: &mut OutputController) {
    output_controller.enter_command(&format!("players {players_count}"));
}

fn set_players_count(g: &mut Game) {
    if g.profile.players_count > 1 {
        _set_players_count(g.profile.players_count, &mut g.output_controller);
    }
}

fn set_no_pickup(output_controller: &mut OutputController) {
    output_controller.enter_command("nopickup");
}

fn _enter_game_with_difficulty(
    difficulty: GameDifficulty,
    output_controller: &mut OutputController,
) {
    output_controller.click_key(enigo::Key::Layout(difficulty.to_keyboard_shortcut_key()));
}

fn enter_game_with_difficulty(g: &mut Game) -> Result<(), WrongGameStateError> {
    _enter_game_with_difficulty(g.profile.game_difficulty, &mut g.output_controller);

    match wait_for_enum_state(
        g,
        State::InGame,
        g.bot_settings.max_frames_to_wait_for_enter_game,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// This function enters the single player menu, selects our character and then enters a game with the desired difficulty level (normal, nightmare or hell)
fn enter_game(g: &mut Game, is_first_run: bool) -> Result<(), WrongGameStateError> {
    enter_single_player_menu(g)?;

    select_character(g, is_first_run);

    let state_id = wait_for_enum_states(g, &[State::DifficultyMenu, State::InGame], 200)?;

    // If our character has not completed the game on normal difficulty yet then the character does not have to choose between normal, nightmare or hell
    // when entering the game. This means that it will already be in the game at this point.
    // Therefore we only perform the action with the difficulty menu if we are not already in the game
    if state_id == 0 {
        // If the state is 0 after selecting the character then we need to select the desired difficulty
        enter_game_with_difficulty(g)?;
    }

    g.belt.reset_potion_consume_time();

    Ok(())
}

fn exit_and_enter_new_game(g: &mut Game) -> Result<(), WrongGameStateError> {
    open_game_menu(g)?;

    save_and_exit_game(g)?;

    enter_game(g, false)?;

    Ok(())
}

fn start(profile_name: &str) -> Result<(), BotError> {
    let file_io = FileIo::new();
    let profile = file_io.load_profile(profile_name).unwrap();
    let system_settings = file_io.load_system_settings().unwrap();
    let bot_settings = file_io.load_bot_settings().unwrap();
    let archives = Archives::new(&system_settings.diablo2_folder_path);

    let (game_screenshotter, output_controller) =
        start_diablo2(&bot_settings, &system_settings.diablo2_folder_path);

    let mut game = Game::new(
        archives,
        file_io,
        profile,
        bot_settings,
        game_screenshotter,
        output_controller,
    )
    .unwrap();

    if game.bot_settings.enable_mouse_movement_program_stopper {
        setup_mouse_movement_program_stopper();
    }

    enter_game(&mut game, true)?;

    let current_act = identify_starting_act(&mut game).unwrap();

    let target_act = game.profile.zone_to_farm.to_act();

    if current_act != target_act {
        go_to_act(&mut game, current_act, target_act);

        exit_and_enter_new_game(&mut game)?;
    }

    game.inventory_table_reserved_cells = get_inventory_table(&mut game);
    game.inventory = game.inventory_table_reserved_cells.clone();

    update_belt(&mut game);

    for i in 0..game.bot_settings.max_game_runs {
        GameInterfaceElementController::activate_element(&mut game, Automap).unwrap();

        GameInterfaceElementController::deactivate_element(&mut game, Portraits)?;

        check_health_and_mana(&mut game)?;

        match run(&mut game, i == 0) {
            Ok(_) => {}
            Err(bot_error) => {
                println!("{}", bot_error);
                match bot_error {
                    BotError::LowHealthManaAndNoPotionInBelt(_) => {
                        exit_and_enter_new_game(&mut game)?;

                        buy_potions(&mut game);
                    }
                    BotError::CharacterHasDied(_) => {
                        // TODO Continue instead of stopping here
                        exit(1);

                        /*
                        def pickup_corpse(self) -> None:
                            for act, point in zip(
                                ACT,
                                (
                                    Point(263, 418),
                                    Point(267, 380),
                                    Point(276, 400),
                                    Point(272, 387),
                                    Point(258, 407)
                                )):

                                if act == self.game_state.current_act:
                                    self.click(point, sleep_after_cursor_movement=True)
                                    break
                            time.sleep(1)
                        */
                    }
                    BotError::WrongGameState(_) => {
                        // TODO Continue instead of stopping here
                        exit(1);
                    }
                    BotError::LowInventorySpace(_) => {
                        exit_and_enter_new_game(&mut game)?;

                        identify_items_at_deckard_cain(&mut game)?;

                        drop_unwanted_items(&mut game);

                        exit_and_enter_new_game(&mut game)?;

                        if move_items_to_stash::move_items_to_stash(&mut game).is_err() {
                            open_game_menu(&mut game)?;
                            save_and_exit_game(&mut game)?;
                            return Ok(());
                        }

                        game.inventory = get_inventory_table(&mut game);
                    }
                    BotError::HealthPointsUnderHardLimit(_)
                    | BotError::MovedToTownZone(_)
                    | BotError::CouldNotGetPath(_) => {}
                    BotError::ArchiveError(archive_error) => {
                        return Err(BotError::ArchiveError(archive_error))
                    }
                }
            }
        }

        open_game_menu(&mut game)?;
        save_and_exit_game(&mut game)?;

        if !is_last_element(i as usize, game.bot_settings.max_game_runs as usize) {
            enter_game(&mut game, false)?;
        }
    }

    Ok(())
}

fn check_health_and_mana(g: &mut Game) -> Result<(), WrongGameStateError> {
    let img = g.game_screenshotter.take_screenshot();
    let matrix = img.to_matrix(&g.palette_transformer);

    match get_points_and_drink_potions_if_points_under_soft_limit(g, &matrix) {
        Ok(drunk_potions) => {
            let mut has_drunk_healing_potion = false;
            let mut has_drunk_mana_potion = false;

            for potion in drunk_potions.iter() {
                match potion {
                    HealthManaPotionType::HealingPotion => {
                        has_drunk_healing_potion = true;
                    }
                    HealthManaPotionType::ManaPotion => {
                        has_drunk_mana_potion = true;
                    }
                    _ => {}
                }
            }

            let potion_fill_time_milliseconds = {
                if has_drunk_healing_potion {
                    Some(Milliseconds(7000))
                } else if has_drunk_mana_potion {
                    Some(Milliseconds(5000))
                } else {
                    None
                }
            };

            if let Some(potion_fill_time_milliseconds) = potion_fill_time_milliseconds {
                sleep_millis(potion_fill_time_milliseconds);
            }
        }
        Err(_) => {
            buy_potions(g);

            exit_and_enter_new_game(g)?;
        }
    }

    Ok(())
}

fn save_and_exit_game(g: &mut Game) -> Result<(), WrongGameStateError> {
    g.output_controller
        .click_mouse(SAVE_AND_EXIT_GAME_POINT, ClickType::Left, true, true);

    wait_for_enum_state(
        g,
        State::MainMenu,
        g.bot_settings.max_frames_to_wait_for_exit_game,
    )?;

    g.game_interface_element_controller =
        GameInterfaceElementController::new(&g.profile.keybindings.game_interface_actions);

    Ok(())
}

fn find_character_name(texts: Vec<Item>, character_name: &str) -> Option<PointU16> {
    for text in texts {
        if text.name == character_name {
            return Some(text.point);
        }
    }

    None
}

fn select_character(g: &mut Game, is_first_run: bool) {
    let character_screen_point = match is_first_run {
        true => {
            g.output_controller.move_mouse_to_safe_point();

            let img = g.game_screenshotter.take_screenshot();

            let font_char_map = get_font_char_map(g);

            let text = match_unique_text_with_palette(
                g,
                &font_char_map,
                &img,
                enums::palette::Palette::Sky,
            );

            find_character_name(text, &g.profile.character_name)
        }
        false => Some(TOP_LEFT_CHARACTER_POINT), // The last played character will always be placed in the top left cell,
    }
    .expect("Should find character name");

    g.output_controller
        .double_click(character_screen_point, ClickType::Left, true, true);
}

fn _get_game_window_location(
    full_screen: &Image,
    validation_pixels: &[ValidationPixel],
) -> Option<PointU16> {
    for row in 0..full_screen.dims.row - GAME_WINDOW_SIZE.row {
        for col in 0..(full_screen.dims.col - GAME_WINDOW_SIZE.col) {
            let mut is_row_match = true;

            for validation_pixel in validation_pixels {
                if full_screen.pixels[(usize::from(row) + validation_pixel.point.row as usize)
                    * full_screen.dims.col as usize
                    + usize::from(col)
                    + validation_pixel.point.col as usize]
                    != validation_pixel.pixel
                {
                    is_row_match = false;
                    break;
                }
            }

            if is_row_match {
                return Some(PointU16 { row, col });
            }
        }
    }

    None
}

fn get_game_window_location(
    bot_settings: &BotSettings,
    is_started_game_from_script: bool,
) -> (GameScreenshotter, OutputController) {
    let mut screenshotter = Screenshotter::new();

    let validation_pixels = if is_started_game_from_script {
        &START_SCREEN_LOADING_SCREEN_VALIDATION_PIXELS
    } else {
        &EXIT_GAME_VALIDATION_PIXELS
    };

    let mut enigo = Enigo::new();

    // We identify the location of the game window by searching the entire screen for a specific part of the game window.
    // To avoid the cursor overlapping that part of the game window we first move it out of the way.
    enigo.mouse_move_to(1, 1);

    for _ in 0..bot_settings.max_frames_to_wait_for_locate_game_window {
        let full_screen = screenshotter.take_full_screenshot();
        match _get_game_window_location(&full_screen, validation_pixels) {
            Some(offset_point) => {
                let window_size = GAME_WINDOW_SIZE;

                let mut output_controller = OutputController::new(
                    enigo,
                    offset_point,
                    window_size,
                    PointU16 { row: 0, col: 0 },
                    bot_settings.clone(),
                );

                if is_started_game_from_script {
                    output_controller.click_mouse(
                        PointU16::new(500, 700),
                        ClickType::Left,
                        true,
                        true,
                    );
                    sleep_frame();
                }

                let mut game_screenshotter =
                    GameScreenshotter::new(screenshotter, offset_point, window_size);

                wait_for_enum_state_(
                    &mut game_screenshotter,
                    State::MainMenu,
                    bot_settings.max_frames_to_wait_for_exit_game,
                )
                .unwrap();

                return (game_screenshotter, output_controller);
            }
            None => {
                sleep_frame();
            }
        }
    }

    panic!("Could not find game window!");
}

fn get_waypoint_point(g: &mut Game) -> PointU16 {
    g.output_controller.move_mouse_to_safe_point();

    let matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    let texts = g.font_symbol_matcher.match_image_items(&matrix);

    for text in &texts {
        if text.name == g.profile.zone_to_farm.to_string() {
            if text.quality == Quality::Grey {
                println!("Your character does not have the waypoint to your specified farm zone '{}' Exiting program!", g.profile.zone_to_farm);
                exit(1);
            } else if text.quality == Quality::Common {
                return text.point;
            }
        }
    }

    println!(
        "Could not find the waypoint text of your specified farm zone '{}' Exiting program!",
        g.profile.zone_to_farm
    );
    exit(1);
}

fn waypoint_to_zone(g: &mut Game) {
    g.output_controller
        .ensure_mouse_is_out_of_area(WAYPOINT_TEXT_AREA);

    let waypoint_point = get_waypoint_point(g) + PointU16::new(10, 0);

    g.output_controller
        .click_mouse(waypoint_point, ClickType::Left, true, true);
    g.output_controller.move_mouse_to_safe_point();

    wait_while_in_enum_state(
        g,
        State::WaypointMenu,
        g.bot_settings.max_frames_to_wait_for_zone_load,
    )
    .expect("Could not enter zone from waypoint!");
}

fn _walk_to_waypoint(g: &mut Game, act: Act) {
    GameInterfaceElementController::activate_element(g, Automap).unwrap();

    let waypoint_route = get_town_waypoint_route(g, act).unwrap();

    walk_enum_route_with_end_state(g, waypoint_route, State::WaypointMenu).unwrap();
}

fn walk_to_waypoint(g: &mut Game) {
    _walk_to_waypoint(g, g.profile.zone_to_farm.to_act());
}

fn get_town_waypoint_route(g: &mut Game, act: Act) -> Option<Route> {
    match act {
        Act::Act1 => get_act_1_town_waypoint_route(g),
        Act::Act2 => Some(Route::Act2StartToWaypoint),
        Act::Act3 => Some(Route::Act3StartToWaypoint),
        Act::Act4 => Some(Route::Act4StartToWaypoint),
        Act::Act5 => Some(Route::Act5StartToWaypoint),
    }
}

fn get_act_1_town_waypoint_route(g: &mut Game) -> Option<Route> {
    let img = g.game_screenshotter.take_screenshot();
    _get_act_1_town_waypoint_route(&img)
}

fn _get_act_1_town_waypoint_route(img: &Image) -> Option<Route> {
    let pixels = [
        Pixel {
            red: 144,
            green: 184,
            blue: 252,
        },
        Pixel {
            red: 168,
            green: 204,
            blue: 252,
        },
        Pixel {
            red: 36,
            green: 96,
            blue: 216,
        },
    ];

    let points_routes = [
        (PointU16 { row: 247, col: 420 }, Route::Act1StartToWaypoint1),
        (PointU16 { row: 287, col: 468 }, Route::Act1StartToWaypoint2),
        (PointU16 { row: 279, col: 468 }, Route::Act1StartToWaypoint3),
        (PointU16 { row: 243, col: 412 }, Route::Act1StartToWaypoint4),
    ];

    for (point, route) in points_routes {
        let is_point_0_match = img.get_value(point) == pixels[0];
        let is_point_1_match = img.get_value(PointU16::new(point.row + 1, point.col)) == pixels[1];
        let is_point_2_match = img.get_value(PointU16::new(point.row + 2, point.col)) == pixels[2];

        if is_point_0_match && is_point_1_match && is_point_2_match {
            return Some(route);
        }
    }

    None
}

impl Pixel {
    fn get_flat_index(self) -> usize {
        ((((u32::from(self.red)) << 10) + ((u32::from(self.green)) << 4) + (u32::from(self.blue)))
            >> 2) as usize
    }
}

fn take_screenshot(g: &mut Game) -> Matrix {
    g.output_controller.move_mouse_to_safe_point();

    g.game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer)
}

fn is_moving(g: &mut Game) -> bool {
    let img1 = g.game_screenshotter.take_screenshot();
    sleep_frame();
    let img2 = g.game_screenshotter.take_screenshot();
    sleep_frame();

    let diff_percentage = img1.get_diff_percentage(&img2);
    let has_moving_diff = diff_percentage > 0.2; // While moving most of the screen will change between each frame. While standing still most of the screen will stay the same.
    let has_minimal_diff = diff_percentage < 0.001; // Such a small difference between frames might be caused by lag and we therefore assume that we are still moving.

    has_moving_diff || has_minimal_diff
}

fn wait_while_moving(g: &mut Game) {
    let max_count = 200;

    for _ in 0..max_count {
        if !is_moving(g) {
            return;
        }
    }

    panic!("Still moving after {max_count} checks!");
}

fn do_first_run_actions(g: &mut Game) -> Result<(), WrongGameStateError> {
    if g.profile.set_game_options {
        set_video_options(g)?;
        set_automap_options(g)?;
    }

    ensure_health_and_mana_text_is_toggled_on(g);
    update_belt(g);
    set_no_pickup(&mut g.output_controller);
    set_players_count(g);

    Ok(())
}

fn update_belt(g: &mut Game) {
    g.output_controller.move_mouse_to_safe_point();

    let img = GameInterfaceElementController::activate_element(g, Belt)
        .unwrap()
        .unwrap();

    let matrix = img.to_matrix(&g.palette_transformer);

    GameInterfaceElementController::deactivate_element(g, Belt).unwrap(); // TODO Throw error

    g.belt.update_belt_and_remove_unneeded_potions(
        &matrix,
        &g.consumable_items_table_matcher,
        &mut g.output_controller,
    );
}

fn read_current_zone(g: &Game, matrix: &Matrix) -> Option<String> {
    let zone_name_area = matrix.get_sub_matrix2(ZONE_NAME_AREA);

    let mut items = g.font_symbol_matcher.match_image_items(&zone_name_area);
    items.sort_by(|a, b| a.point.cmp(&b.point));

    for item in items {
        if is_valid_zone(&item.name) {
            return Some(item.name);
        }
    }

    None
}

fn change_level(g: &mut Game, level: &str) -> Result<(), MovedToTownZoneError> {
    let update_level = if TOWN_LEVELS.contains(&level) {
        return Err(MovedToTownZoneError);
    } else {
        level != g.monster_matcher.level
    };

    if update_level {
        let act = g.profile.zone_to_farm.to_act();

        let level_monsters_matcher_config = MonsterMatcherConfig::new_levels_monster_matcher_config(
            act,
            g.profile.game_difficulty,
            &g.bot_settings,
        );

        g.monster_matcher = get_monster_tree(
            &mut g.archives,
            &g.file_io,
            level,
            &level_monsters_matcher_config,
            &g.zone_name_converter,
        )
        .unwrap();
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let profile_name = &args[1];

    match start(profile_name.as_str()) {
        Ok(_) => println!("Finishing bot run"),
        Err(e) => println!("Stoping bot due to error: {:?}", e),
    }
}
