use crate::{
    constants::{
        game_window_areas::NOISY_AREAS,
        validation_pixels::{
            AUTOMAP_FADE_NO_VALIDATION_PIXELS, AUTOMAP_OPTIONS_MENU_VALIDATION_PIXELS,
            AUTOMAP_SHOW_PARTY_NO_VALIDATION_PIXELS, AUTOMAP_SIZE_FULL_VALIDATION_PIXELS,
            BELT_OPEN_VALIDATION_PIXELS, DIFFICULTY_MENU_VALIDATION_PIXELS,
            EXIT_GAME_VALIDATION_PIXELS, HAS_DIED_VALIDATION_PIXELS,
            INVENTORY_OPEN_VALIDATION_PIXELS, IN_GAME_VALIDATION_PIXELS,
            LIGHTING_QUALITY_LOW_VALIDATION_PIXELS, MENU_VALIDATION_PIXELS,
            MERCHANT_TRADE_WINDOW_OPEN_VALIDATION_PIXELS, OPTIONS_MENU_VALIDATION_PIXELS,
            SINGLE_PLAYER_MENU_VALIDATION_PIXELS, STASH_VALIDATION_PIXELS,
            VIDEO_OPTIONS_MENU_VALIDATION_PIXELS, WAYPOINT_MENU_VALIDATION_PIXELS,
        },
    },
    enums::{errors::WrongGameStateError, state::State},
    game::Game,
    game_screenshotter::GameScreenshotter,
    image::Image,
    matrix::Matrix,
    point_u16::PointU16,
    structs::Pixel,
    utils::sleep_frame,
};

pub struct ValidationPixel {
    pub point: PointU16,
    pub pixel: Pixel,
}

fn check_state(img: &Image, validation_pixels: &[ValidationPixel]) -> bool {
    let mut valid_count = 0;

    for validation_pixel in validation_pixels {
        if img.get_value(validation_pixel.point) == validation_pixel.pixel {
            valid_count += 1;
        }
    }

    (valid_count as f32 / validation_pixels.len() as f32) > 0.8
}

fn get_state_validation_pixels(state: State) -> &'static [ValidationPixel] {
    match state {
        State::AutomapFadeNo => &AUTOMAP_FADE_NO_VALIDATION_PIXELS,
        State::AutomapOptionsMenu => &AUTOMAP_OPTIONS_MENU_VALIDATION_PIXELS,
        State::AutomapShowPartyNo => &AUTOMAP_SHOW_PARTY_NO_VALIDATION_PIXELS,
        State::AutomapSizeFull => &AUTOMAP_SIZE_FULL_VALIDATION_PIXELS,
        State::DifficultyMenu => &DIFFICULTY_MENU_VALIDATION_PIXELS,
        State::MainMenu => &EXIT_GAME_VALIDATION_PIXELS,
        State::HasDied => &HAS_DIED_VALIDATION_PIXELS,
        State::InGame => &IN_GAME_VALIDATION_PIXELS,
        State::LightingQualityLow => &LIGHTING_QUALITY_LOW_VALIDATION_PIXELS,
        State::Menu => &MENU_VALIDATION_PIXELS,
        State::OptionsMenu => &OPTIONS_MENU_VALIDATION_PIXELS,
        State::SinglePlayerMenu => &SINGLE_PLAYER_MENU_VALIDATION_PIXELS,
        State::Stash => &STASH_VALIDATION_PIXELS,
        State::VideoOptionsMenu => &VIDEO_OPTIONS_MENU_VALIDATION_PIXELS,
        State::WaypointMenu => &WAYPOINT_MENU_VALIDATION_PIXELS,
        State::InventoryOpen => &INVENTORY_OPEN_VALIDATION_PIXELS,
        State::BeltOpen => &BELT_OPEN_VALIDATION_PIXELS,
        State::MerchantTradeWindowOpen => &MERCHANT_TRADE_WINDOW_OPEN_VALIDATION_PIXELS,
    }
}

fn is_in_state(img: &Image, state: State) -> bool {
    let state_validation_pixels = get_state_validation_pixels(state);
    check_state(img, state_validation_pixels)
}

pub fn is_in_enum_state(state: State, img: &Image) -> bool {
    is_in_state(img, state)
}

pub fn wait_for_enum_states(
    g: &mut Game,
    states: &[State],
    max_frame_wait_count: u32,
) -> Result<u32, WrongGameStateError> {
    for _ in 0..max_frame_wait_count {
        let img = g.game_screenshotter.take_screenshot();

        for (i, state) in states.iter().enumerate() {
            if is_in_enum_state(*state, &img) {
                return Ok(i as u32);
            }
        }

        sleep_frame();
    }

    Err(WrongGameStateError::new(
        &g.game_screenshotter.take_screenshot(),
    ))
}

pub fn wait_for_merchant_dialog_menu(
    g: &mut Game,
    max_frame_wait_count: u32,
) -> Result<Matrix, WrongGameStateError> {
    for _ in 0..max_frame_wait_count {
        let mut matrix = g
            .game_screenshotter
            .take_screenshot()
            .to_matrix(&g.palette_transformer);

        matrix.clear_areas(&NOISY_AREAS);

        let items = g.font_symbol_matcher.match_image_items(&matrix);
        for item in items {
            if item.name == "talk" {
                return Ok(matrix);
            }
        }

        sleep_frame();
    }

    Err(WrongGameStateError::new(
        &g.game_screenshotter.take_screenshot(),
    ))
}

pub fn wait_while_merchant_dialog_menu_open(
    g: &mut Game,
    max_frame_wait_count: u32,
) -> Result<Matrix, WrongGameStateError> {
    for _ in 0..max_frame_wait_count {
        let mut matrix = g
            .game_screenshotter
            .take_screenshot()
            .to_matrix(&g.palette_transformer);

        matrix.clear_areas(&NOISY_AREAS);

        let items = g.font_symbol_matcher.match_image_items(&matrix);

        let mut found_dialog_option = false;

        for item in items {
            if item.name == "talk" {
                found_dialog_option = true;
            }
        }

        if !found_dialog_option {
            return Ok(matrix);
        }

        sleep_frame();
    }

    Err(WrongGameStateError::new(
        &g.game_screenshotter.take_screenshot(),
    ))
}

pub fn wait_for_enum_state(
    g: &mut Game,
    state: State,
    max_frame_wait_count: u32,
) -> Result<Image, WrongGameStateError> {
    wait_for_enum_state_(&mut g.game_screenshotter, state, max_frame_wait_count)
}

pub fn wait_for_enum_state_(
    game_screenshotter: &mut GameScreenshotter,
    state: State,
    max_frame_wait_count: u32,
) -> Result<Image, WrongGameStateError> {
    for _ in 0..max_frame_wait_count {
        let img = game_screenshotter.take_screenshot();

        if is_in_enum_state(state, &img) {
            return Ok(img);
        }

        sleep_frame();
    }

    Err(WrongGameStateError::new(
        &game_screenshotter.take_screenshot(),
    ))
}

pub fn wait_while_in_enum_state(
    g: &mut Game,
    state: State,
    max_frame_wait_count: u32,
) -> Result<Image, WrongGameStateError> {
    for _ in 0..max_frame_wait_count {
        let img = g.game_screenshotter.take_screenshot();

        if !is_in_enum_state(state, &img) {
            return Ok(img);
        }

        sleep_frame();
    }

    Err(WrongGameStateError::new(
        &g.game_screenshotter.take_screenshot(),
    ))
}
