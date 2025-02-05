use std::{collections::HashSet, process::exit};

use crate::{
    constants::game_window_points::INVENTORY_GOLD_POINT,
    enums::{click_type::ClickType, errors::LowStashSpaceError, state::State},
    game::Game,
    point_u16::PointU16,
    route_walker::walk_enum_route_with_end_state,
    state_validator::{wait_for_enum_state, wait_while_in_enum_state},
    table::Table,
    utils::sleep_frames,
};

pub fn move_items_to_stash(g: &mut Game) -> Result<(), LowStashSpaceError> {
    let route = g.profile.zone_to_farm.to_act().get_stash_route();

    walk_enum_route_with_end_state(g, route, State::Stash).unwrap(); // TODO Throw error

    place_items_in_stash(g)
}

fn place_items_in_stash(g: &mut Game) -> Result<(), LowStashSpaceError> {
    g.output_controller.move_mouse_to_safe_point();
    wait_for_enum_state(
        g,
        State::Stash,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    )
    .unwrap(); // TODO Throw error

    let matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    let mut inventory_before_item_pickup =
        g.table_inventory_empty_matcher.match_from_matrix(&matrix);

    let mut stash = g.table_stash_empty_matcher.match_from_matrix(&matrix);
    let stash_table_meta_data = g.table_stash_empty_matcher.get_table_meta_data();
    let inventory_table_meta_data = g.table_inventory_empty_matcher.get_table_meta_data();

    let mut no_more_space_in_stash = false;

    for row in 0..inventory_table_meta_data.table_size.row as usize {
        for col in 0..inventory_table_meta_data.table_size.col as usize {
            if g.inventory_table_reserved_cells.cells[row][col].is_none()
                && inventory_before_item_pickup.cells[row][col].is_some()
            {
                // Pickup item from iventory
                let item_screen_point = inventory_table_meta_data
                    .get_point(PointU16::new(row as u16, col as u16))
                    + PointU16::new(5, 5);

                g.output_controller
                    .click_mouse(item_screen_point, ClickType::Left, true, true);

                g.output_controller.move_mouse_to_safe_point();
                sleep_frames(g.bot_settings.stash_settings.num_frames_to_sleep_after_picking_up_item_from_inventory_before_moving_it_to_stash);

                // Take new screenshot and compare with the old screenshot to get the size of the item we picked up
                let matrix = g
                    .game_screenshotter
                    .take_screenshot()
                    .to_matrix(&g.palette_transformer);
                let inventory_after_item_pickup =
                    g.table_inventory_empty_matcher.match_from_matrix(&matrix);
                let item_size =
                    get_item_size(&inventory_before_item_pickup, &inventory_after_item_pickup);

                if item_size.row == 0 && item_size.col == 0 {
                    exit(1);
                }

                // Find where we can place the item in the stash
                let stash_placement = stash.find_item_placement_in_cell_area(
                    u32::from(item_size.row),
                    u32::from(item_size.col),
                );
                match stash_placement {
                    Some(stash_placement) => {
                        //Move the item to the stash

                        // When I have an item that is more than one cell large
                        // then the mouse is placed in the middle of the item, which then means that the top left corner
                        // may overlap with another item
                        // Therefore we offset the row and col
                        let cell_screen_position = stash_table_meta_data.get_point(stash_placement)
                            + PointU16::new(item_size.row * 14, item_size.col * 14);

                        g.output_controller.click_mouse(
                            cell_screen_position,
                            ClickType::Left,
                            true,
                            true,
                        );

                        g.output_controller.move_mouse_to_safe_point();
                        sleep_frames(
                            g.bot_settings
                                .stash_settings
                                .num_frames_to_sleep_after_placing_item_in_stash,
                        );

                        let matrix = g
                            .game_screenshotter
                            .take_screenshot()
                            .to_matrix(&g.palette_transformer);
                        inventory_before_item_pickup =
                            g.table_inventory_empty_matcher.match_from_matrix(&matrix);
                        stash = g.table_stash_empty_matcher.match_from_matrix(&matrix);
                    }
                    None => {
                        println!("No more space in stash");
                        println!("Placing item back in inventory");

                        no_more_space_in_stash = true;

                        if let Some(inventory_placement) = inventory_after_item_pickup
                            .find_item_placement_in_cell_area(
                                u32::from(item_size.row),
                                u32::from(item_size.col),
                            )
                        {
                            let cell_screen_position = inventory_table_meta_data
                                .get_point(inventory_placement)
                                + PointU16::new(item_size.row * 14, item_size.col * 14);

                            g.output_controller.click_mouse(
                                cell_screen_position,
                                ClickType::Left,
                                true,
                                true,
                            );

                            sleep_frames(
                                g.bot_settings
                                    .stash_settings
                                    .num_frames_to_sleep_after_placing_item_in_stash,
                            );
                        }

                        g.output_controller.move_mouse_to_safe_point();
                        sleep_frames(
                            g.bot_settings
                                .stash_settings
                                .num_frames_to_sleep_after_placing_item_in_stash,
                        );

                        let matrix = g
                            .game_screenshotter
                            .take_screenshot()
                            .to_matrix(&g.palette_transformer);
                        let inventory = g.table_inventory_empty_matcher.match_from_matrix(&matrix);

                        // Press escape to close the stash
                        if !inventory.has_space_for_item(4, 2) {
                            println!("No more space in inventory");
                            g.output_controller.click_key(enigo::Key::Escape);

                            g.output_controller.move_mouse_to_safe_point();

                            wait_while_in_enum_state(
                                g,
                                State::Stash,
                                g.bot_settings.max_frames_to_wait_for_ui_action,
                            )
                            .unwrap(); // TODO Throw error

                            return Err(LowStashSpaceError);
                        }
                    }
                }
            }

            if no_more_space_in_stash {
                break;
            }
        }
        if no_more_space_in_stash {
            break;
        }
    }

    sleep_frames(
        g.bot_settings
            .stash_settings
            .num_frames_to_sleep_after_placing_items_in_stash,
    );

    // Transfer gold to the stash
    g.output_controller
        .click_mouse(INVENTORY_GOLD_POINT, ClickType::Left, true, true);
    g.output_controller.click_key(enigo::Key::Return);
    sleep_frames(
        g.bot_settings
            .stash_settings
            .num_frames_to_sleep_after_moving_gold_to_stash,
    );

    // Press escape to close the stash
    g.output_controller.click_key(enigo::Key::Escape);

    g.output_controller.move_mouse_to_safe_point();

    wait_while_in_enum_state(
        g,
        State::Stash,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    )
    .unwrap(); // TODO Throw error

    Ok(())
}

fn get_item_size(area_before_item_pickup: &Table, area_after_item_pickup: &Table) -> PointU16 {
    let mut changed_rows = HashSet::new();
    let mut changed_cols = HashSet::new();

    for row in 0..area_before_item_pickup.cells.len() {
        for col in 0..area_before_item_pickup.cells[0].len() {
            let has_item_in_cell_before_pickup = area_before_item_pickup.cells[row][col].is_some();
            let has_item_in_cell_after_pickup = area_after_item_pickup.cells[row][col].is_some();

            if has_item_in_cell_before_pickup != has_item_in_cell_after_pickup {
                changed_rows.insert(row);
                changed_cols.insert(col);
            }
        }
    }

    PointU16 {
        row: changed_rows.len() as u16,
        col: changed_cols.len() as u16,
    }
}
