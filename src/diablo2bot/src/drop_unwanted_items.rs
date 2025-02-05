use crate::{
    enums::{click_type, game_interface_element::GameInterfaceElement},
    game::Game,
    game_interface_element_controller::GameInterfaceElementController,
    loot::{is_item_potion, take_loot_screenshot},
    point_u16::PointU16,
    structs::Item,
    units::Frames,
    utils::sleep_frames,
};

pub fn drop_unwanted_items(g: &mut Game) {
    g.output_controller.move_mouse_to_safe_point();

    let img = GameInterfaceElementController::activate_element(g, GameInterfaceElement::Inventory)
        .unwrap()
        .unwrap();

    let matrix = img.to_matrix(&g.palette_transformer);

    let mut inventory = g.table_inventory_empty_matcher.match_from_matrix(&matrix);

    let inventory_table_meta_data = g.table_inventory_empty_matcher.get_table_meta_data();

    for row in 0..inventory_table_meta_data.table_size.row as usize {
        for col in 0..inventory_table_meta_data.table_size.col as usize {
            if g.inventory_table_reserved_cells.cells[row][col].is_none()
                && inventory.cells[row][col].is_some()
            {
                let item_screen_point = inventory_table_meta_data
                    .get_point(PointU16::new(row as u16, col as u16))
                    + PointU16::new(5, 5);

                g.output_controller.move_mouse(item_screen_point);
                sleep_frames(Frames(2));

                let matrix = take_loot_screenshot(g);
                let items = g.font_symbol_matcher.match_image_items(&matrix);

                if !keep_item(&items) {
                    drop_item(g, item_screen_point);
                }

                g.output_controller.move_mouse_to_safe_point();

                let matrix = g
                    .game_screenshotter
                    .take_screenshot()
                    .to_matrix(&g.palette_transformer);

                inventory = g.table_inventory_empty_matcher.match_from_matrix(&matrix);
            }
        }
    }

    // TODO Throw error
    GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Inventory).unwrap();
}

fn drop_item(g: &mut Game, item_screen_point: PointU16) {
    g.output_controller
        .click_mouse(item_screen_point, click_type::ClickType::Left, true, true);

    sleep_frames(Frames(4));

    g.output_controller.click_mouse(
        PointU16 { row: 300, col: 200 },
        click_type::ClickType::Left,
        true,
        true,
    );

    sleep_frames(Frames(4));
}

fn keep_item(items: &[Item]) -> bool {
    // TODO Expand with more granular filtering.
    for item in items.iter() {
        if is_item_potion(item) {
            return false;
        }
    }

    true
}
