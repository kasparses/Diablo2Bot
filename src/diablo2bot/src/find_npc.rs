use crate::{
    constants::game_window_areas::NOISY_AREAS, enums::game_interface_element::GameInterfaceElement,
    game::Game, game_interface_element_controller::GameInterfaceElementController, matrix::Matrix,
    pattern_matcher_monsters::TreeMatch, point_u16::PointU16, units::Frames, utils::sleep_frames,
};

enum Npc {
    PotionSeller,
    DeckardCain,
}

pub fn find_deckard_cain(g: &mut Game) -> Option<PointU16> {
    find_npc(g, Npc::DeckardCain)
}

pub fn find_potion_seller(g: &mut Game) -> Option<PointU16> {
    find_npc(g, Npc::PotionSeller)
}

fn find_npc(g: &mut Game, npc: Npc) -> Option<PointU16> {
    deactivate_automap(g);

    let (matrix1, matrix2) = get_delayed_screenshots(g);

    let matches = find_matches_with_movement(g, npc, &matrix1, &matrix2);

    matches.first().map(|m| m.window_offset_point)
}

fn deactivate_automap(g: &mut Game) {
    // Deactivate the automap as it can interfere with locating the npc
    GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Automap).unwrap();
    // TODO Throw error
}

fn get_delayed_screenshots(g: &mut Game) -> (Matrix, Matrix) {
    let matrix = take_screenshot(g);

    sleep_frames(Frames(2));

    let matrix2 = take_screenshot(g);

    (matrix, matrix2)
}

fn take_screenshot(g: &mut Game) -> Matrix {
    g.output_controller.move_mouse_to_safe_point();

    let mut matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    matrix.clear_areas(&NOISY_AREAS);

    matrix
}

fn find_matches_with_movement(
    g: &mut Game,
    npc: Npc,
    matrix1: &Matrix,
    matrix2: &Matrix,
) -> Vec<TreeMatch> {
    let matches = find_matches(g, npc, matrix1);

    matches
        .into_iter()
        .filter(|m| {
            let window_1 = matrix1.get_window(m.window_offset_point);
            let window_2 = matrix2.get_window(m.window_offset_point);

            window_1 != window_2
        })
        .collect()
}

fn find_matches(g: &mut Game, npc: Npc, matrix: &Matrix) -> Vec<TreeMatch> {
    let matcher = match npc {
        Npc::PotionSeller => &g.potion_seller_matcher,
        Npc::DeckardCain => &g.deckard_cain_matcher,
    };

    matcher.look_up(matrix)
}
