use crate::{
    constants::town_levels::{town_level_to_act, TOWN_LEVELS},
    enums::{
        act::{Act, ACTS},
        game_interface_element::GameInterfaceElement,
    },
    game::Game,
    game_interface_element_controller::GameInterfaceElementController,
    image::Image,
    match_text_with_palette::{get_font_char_map, match_unique_text_with_palette},
    structs::Item,
    units::Frames,
    utils::sleep_frames,
};

pub fn identify_starting_act(g: &mut Game) -> Option<Act> {
    let img = take_screenshot(g);

    if let Some(current_act) = check_expected_act(g, &img) {
        return Some(current_act);
    }

    check_all_acts(g, &img)
}

fn check_expected_act(g: &mut Game, img: &Image) -> Option<Act> {
    let matrix = img.to_matrix(&g.palette_transformer);

    let text = g.font_symbol_matcher.match_image_items(&matrix);

    if let Some(town_level_name) = find_town_zone_name_in_text(&text) {
        return town_level_to_act(&town_level_name);
    }

    None
}

fn check_all_acts(g: &mut Game, img: &Image) -> Option<Act> {
    let font_char_map = get_font_char_map(g);

    for act in ACTS {
        let text = match_unique_text_with_palette(g, &font_char_map, img, act.into());

        if let Some(town_level_name) = find_town_zone_name_in_text(&text) {
            return town_level_to_act(&town_level_name);
        }
    }

    None
}

fn take_screenshot(g: &mut Game) -> Image {
    GameInterfaceElementController::activate_element(g, GameInterfaceElement::Automap).unwrap();

    sleep_frames(Frames(2));

    g.game_screenshotter.take_screenshot()
}

fn find_town_zone_name_in_text(text: &[Item]) -> Option<String> {
    for t in text {
        if TOWN_LEVELS.contains(&t.name.as_str()) {
            return Some(t.name.to_string());
        }
    }

    None
}
