use crate::{
    constants::game_window_areas::{
        HEALTH_GLOBE_AREA, LIFE_TEXT_AREA, MANA_GLOBE_AREA, MANA_TEXT_AREA,
    },
    enums::click_type::ClickType,
    font_matcher::FontMatcher,
    game::Game,
    matrix::Matrix,
};

// Toogles the text above our health and mana globes
// Above the health globe it says "Life <current amount of hp>" and above the mana globe it says "Mana <current amount of mana>"
// This needs to be toogled as we use to to read our health and mana numbers.
pub fn ensure_health_and_mana_text_is_toggled_on(g: &mut Game) {
    g.output_controller.move_mouse_to_safe_point();

    let matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    let is_heath_text_toogled_on = _is_heath_text_toogled_on(&matrix, &g.font_symbol_matcher);
    let is_mana_text_toogled_on = _is_mana_text_toogled_on(&matrix, &g.font_symbol_matcher);

    if !is_heath_text_toogled_on || !is_mana_text_toogled_on {
        g.output_controller.hold_key(enigo::Key::Control);

        if !is_heath_text_toogled_on {
            let point = HEALTH_GLOBE_AREA.get_middle_point();
            g.output_controller
                .click_mouse(point, ClickType::Left, true, true);
        }

        if !is_mana_text_toogled_on {
            let point = MANA_GLOBE_AREA.get_middle_point();
            g.output_controller
                .click_mouse(point, ClickType::Left, true, true);
        }

        g.output_controller.release_key(enigo::Key::Control);
    }
}

fn _is_heath_text_toogled_on(matrix: &Matrix, font_symbol_matcher: &FontMatcher) -> bool {
    let health_text_area_matrix = matrix.get_sub_matrix2(LIFE_TEXT_AREA);
    let health_text = font_symbol_matcher.match_image_items(&health_text_area_matrix);

    for item in health_text {
        if item.name.contains("ife: ") {
            return true;
        }
    }

    false
}

fn _is_mana_text_toogled_on(matrix: &Matrix, font_symbol_matcher: &FontMatcher) -> bool {
    let mana_text_area_matrix = matrix.get_sub_matrix2(MANA_TEXT_AREA);
    let mana_text = font_symbol_matcher.match_image_items(&mana_text_area_matrix);

    for item in mana_text {
        if item.name.contains("Mana: ") {
            return true;
        }
    }

    false
}
