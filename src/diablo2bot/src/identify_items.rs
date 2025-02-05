use crate::{
    buy_potions::find_merchant_dialog_option_point,
    enums::{click_type::ClickType, errors::WrongGameStateError},
    find_npc::find_deckard_cain,
    game::Game,
    point_u16::PointU16,
    route_walker::walk_enum_route_without_end_state,
    state_validator::{wait_for_merchant_dialog_menu, wait_while_merchant_dialog_menu_open},
    units::Frames,
    utils::sleep_frames,
};

pub fn identify_items_at_deckard_cain(g: &mut Game) -> Result<(), WrongGameStateError> {
    walk_to_deckard_cain(g);

    match find_deckard_cain(g) {
        Some(deckard_cain_point) => {
            click_on_deckard_cain(g, deckard_cain_point);

            identify_items(g)?;

            exit_dialog_menu(g);
            Ok(())
        }
        None => {
            // TODO Throw error
            println!("Could not find Deckard Cain :(");
            Ok(())
        }
    }
}

fn walk_to_deckard_cain(g: &mut Game) {
    walk_enum_route_without_end_state(g, g.profile.zone_to_farm.to_act().get_deckard_cain_route());
}

fn click_on_deckard_cain(g: &mut Game, deckard_cain_point: PointU16) {
    g.output_controller
        .click_mouse(deckard_cain_point, ClickType::Left, true, true);

    g.output_controller.move_mouse_to_safe_point();
}

fn identify_items(g: &mut Game) -> Result<(), WrongGameStateError> {
    let merchant_dialog_matrix = wait_for_merchant_dialog_menu(g, 100)?;

    if let Some(identify_items_point) =
        find_merchant_dialog_option_point(g, &merchant_dialog_matrix, "Identify Items")
    {
        g.output_controller
            .click_mouse(identify_items_point, ClickType::Left, true, true);
    }

    Ok(())
}

fn exit_dialog_menu(g: &mut Game) {
    sleep_frames(Frames(10));

    // Press escape to close the dialog menu
    g.output_controller.click_key(enigo::Key::Escape);

    wait_while_merchant_dialog_menu_open(g, 100).unwrap(); // TODO Throw error
}
