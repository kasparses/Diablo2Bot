use crate::{
    constants::game_window_points::{
        AUTOMAP_OPTIONS_MENU_POINT, OPTIONS_MENU_POINT, SINGLE_PLAYER_MENU_POINT,
        SWITCH_AUTOMAP_FADE_POINT, SWITCH_AUTOMAP_SHOW_PARTY_POINT, SWITCH_AUTOMAP_SIZE_POINT,
        SWITCH_LIGHTING_QUALITY_POINT, VIDEO_OPTIONS_MENU_POINT,
    },
    enums::{
        click_type::ClickType::Left,
        errors::WrongGameStateError,
        state::State::{self, AutomapOptionsMenu, OptionsMenu, SinglePlayerMenu},
    },
    game::Game,
    game_screenshotter::GameScreenshotter,
    output_controller::OutputController,
    state_validator::{is_in_enum_state, wait_for_enum_state},
    utils::sleep_frame,
};

// Set Lighting Quality to LOW. This will remove the rain effect which will make it easier to detect monsters.
pub fn set_video_options(g: &mut Game) -> Result<(), WrongGameStateError> {
    open_game_menu(g)?;

    enter_options_menu(g)?;

    enter_video_menu(g);

    let img = g.game_screenshotter.take_screenshot();

    let is_lighting_quality_low = is_in_enum_state(State::LightingQualityLow, &img);

    if !is_lighting_quality_low {
        do_action_and_validate(
            || switch_lighting_quality(&mut g.output_controller),
            State::LightingQualityLow,
            20,
            2,
            &mut g.game_screenshotter,
        )?;
    }

    g.output_controller.click_key(enigo::Key::Escape);
    sleep_frame();

    Ok(())
}

pub fn set_automap_options(g: &mut Game) -> Result<(), WrongGameStateError> {
    open_game_menu(g)?;

    enter_options_menu(g)?;

    enter_automap_menu(g)?;

    let img = g.game_screenshotter.take_screenshot();

    let is_automap_size_full = is_in_enum_state(State::AutomapSizeFull, &img);
    let is_automap_fade_no = is_in_enum_state(State::AutomapFadeNo, &img);
    let is_automap_show_party_no = is_in_enum_state(State::AutomapShowPartyNo, &img);

    if !is_automap_size_full {
        switch_automap_size(&mut g.output_controller);
        sleep_frame();
    }

    if !is_automap_show_party_no {
        switch_automap_show_party(&mut g.output_controller);
        sleep_frame();
    }

    if !is_automap_fade_no {
        do_action_and_validate(
            || switch_automap_fade(&mut g.output_controller),
            State::AutomapFadeNo,
            20,
            2,
            &mut g.game_screenshotter,
        )?;
    }

    g.output_controller.click_key(enigo::Key::Escape);
    sleep_frame();

    Ok(())
}

pub fn open_game_menu(g: &mut Game) -> Result<(), WrongGameStateError> {
    // If we hold alt while pressing escape it will minimize the game window instead of opening up the game menu. Therefore we make sure we are not currently holding alt down.
    g.output_controller.release_key(enigo::Key::Alt);

    do_action_and_validate(
        || _open_game_menu(&mut g.output_controller),
        State::Menu,
        g.bot_settings.max_frames_to_wait_for_ui_action,
        4,
        &mut g.game_screenshotter,
    )
}

fn _open_game_menu(output_controller: &mut OutputController) {
    output_controller.click_key(enigo::Key::Escape);
}

pub fn enter_single_player_menu(g: &mut Game) -> Result<(), WrongGameStateError> {
    g.output_controller
        .click_mouse(SINGLE_PLAYER_MENU_POINT, Left, true, true);

    match wait_for_enum_state(
        g,
        SinglePlayerMenu,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn enter_options_menu(g: &mut Game) -> Result<(), WrongGameStateError> {
    g.output_controller
        .click_mouse(OPTIONS_MENU_POINT, Left, true, true);

    match wait_for_enum_state(
        g,
        OptionsMenu,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn enter_automap_menu(g: &mut Game) -> Result<(), WrongGameStateError> {
    g.output_controller
        .click_mouse(AUTOMAP_OPTIONS_MENU_POINT, Left, true, true);

    match wait_for_enum_state(
        g,
        AutomapOptionsMenu,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn enter_video_menu(g: &mut Game) {
    g.output_controller
        .click_mouse(VIDEO_OPTIONS_MENU_POINT, Left, true, true);
    wait_for_enum_state(
        g,
        State::VideoOptionsMenu,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    )
    .unwrap(); // TODO Throw error
}

fn switch_automap_size(output_controller: &mut OutputController) {
    output_controller.click_mouse(SWITCH_AUTOMAP_SIZE_POINT, Left, true, true);
}

fn switch_automap_show_party(output_controller: &mut OutputController) {
    output_controller.click_mouse(SWITCH_AUTOMAP_SHOW_PARTY_POINT, Left, true, true);
}

fn switch_automap_fade(output_controller: &mut OutputController) {
    output_controller.click_mouse(SWITCH_AUTOMAP_FADE_POINT, Left, true, true);
}

fn switch_lighting_quality(output_controller: &mut OutputController) {
    output_controller.click_mouse(SWITCH_LIGHTING_QUALITY_POINT, Left, true, true);
}

fn do_action_and_validate<F>(
    mut action: F,
    state: State,
    max_frame_wait_count: u32,
    repeat_frequency: u32,
    game_screenshotter: &mut GameScreenshotter,
) -> Result<(), WrongGameStateError>
where
    F: FnMut(),
{
    for i in 0..max_frame_wait_count {
        if i == 0 || i % repeat_frequency == 0 {
            action();
        }

        sleep_frame();

        let img = game_screenshotter.take_screenshot();
        let is_in_state = is_in_enum_state(state, &img);

        if is_in_state {
            return Ok(());
        }
    }

    Err(WrongGameStateError::new(
        &game_screenshotter.take_screenshot(),
    ))
}
