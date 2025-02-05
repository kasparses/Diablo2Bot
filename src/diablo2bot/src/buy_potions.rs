use std::collections::HashMap;

use crate::{
    constants::{
        potion_rankings::{HEALING_POTION_RANKINGS, MANA_POTION_RANKINGS},
        table_meta_data::MERCHANT_TABLE_META_DATA,
    },
    enums::{click_type::ClickType, state::State},
    find_npc::find_potion_seller,
    game::Game,
    matrix::Matrix,
    point_u16::PointU16,
    route_walker::walk_enum_route_without_end_state,
    state_validator::{
        wait_for_enum_state, wait_for_merchant_dialog_menu, wait_while_in_enum_state,
    },
    utils::sleep_frames,
};

enum PotionType {
    Healing,
    Mana,
}

pub fn buy_potions(g: &mut Game) {
    walk_to_potion_seller(g);

    click_on_potion_seller(g);

    buy_potions_from_merchant(g);

    close_trade_window(g);
}

fn walk_to_potion_seller(g: &mut Game) {
    let route = g.profile.zone_to_farm.to_act().get_potion_seller_route();

    walk_enum_route_without_end_state(g, route);
}

fn close_trade_window(g: &mut Game) {
    g.output_controller.click_key(enigo::Key::Escape);
    wait_while_in_enum_state(g, State::MerchantTradeWindowOpen, 200).unwrap(); // TODO Throw error
}

fn buy_potions_from_merchant(g: &mut Game) {
    g.output_controller.move_mouse_to_safe_point();

    let matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    let table = g
        .consumable_items_table_matcher
        .match_from_matrix(&matrix, MERCHANT_TABLE_META_DATA);

    let best_healing_potion = get_best_potion(&table.cells, &PotionType::Healing).unwrap(); // TODO Throw error
    let best_mana_potion = get_best_potion(&table.cells, &PotionType::Mana).unwrap(); // TODO Throw error

    for (cell, potion_name) in &[best_healing_potion, best_mana_potion] {
        loop {
            let has_space_for_belt = g.belt.auto_add_item(potion_name);

            if !has_space_for_belt {
                break;
            }

            let item_screen_point = MERCHANT_TABLE_META_DATA.get_point(*cell) + PointU16::new(5, 5);

            g.output_controller
                .click_mouse(item_screen_point, ClickType::Right, true, true);
            sleep_frames(g.bot_settings.merchant_purchase_cooldown_frames);
        }
    }
}

fn get_best_potion(
    items: &[Vec<Option<String>>],
    potion_type: &PotionType,
) -> Option<(PointU16, String)> {
    let mut best_potion: Option<(PointU16, String)> = None;
    let mut best_potion_ranking = -1;

    let potions_rankings: HashMap<&str, i32> = match potion_type {
        PotionType::Healing => HEALING_POTION_RANKINGS.into_iter().collect(),
        PotionType::Mana => MANA_POTION_RANKINGS.into_iter().collect(),
    };

    for (row, item_row) in items.iter().enumerate() {
        for (col, item) in item_row.iter().enumerate() {
            if let Some(item_name) = &item {
                if let Some(&rank) = potions_rankings.get(item_name.as_str()) {
                    if rank > best_potion_ranking {
                        best_potion_ranking = rank;
                        best_potion =
                            Some((PointU16::new(row as u16, col as u16), item_name.clone()));
                    }
                }
            }
        }
    }

    best_potion
}

fn click_on_potion_seller(g: &mut Game) {
    let merchant_point = find_potion_seller(g).unwrap(); // TODO Throw error

    g.output_controller
        .click_mouse(merchant_point, ClickType::Left, true, false);

    g.output_controller.move_mouse_to_safe_point();

    let merchant_dialog_matrix = wait_for_merchant_dialog_menu(g, 100).unwrap(); // TODO Throw error

    let trade_option_point =
        find_merchant_dialog_option_point(g, &merchant_dialog_matrix, "trade").unwrap(); // TODO Throw error

    g.output_controller
        .click_mouse(trade_option_point, ClickType::Left, true, false);

    wait_for_enum_state(
        g,
        State::MerchantTradeWindowOpen,
        g.bot_settings.max_frames_to_wait_for_ui_action,
    )
    .unwrap(); // TODO Throw error
}

pub fn find_merchant_dialog_option_point(
    g: &Game,
    matrix: &Matrix,
    dialog_option_text: &str,
) -> Option<PointU16> {
    let items = g.font_symbol_matcher.match_image_items(matrix);

    for item in items {
        if item.name == dialog_option_text {
            return Some(item.point + PointU16::new(7, 7));
        }
    }

    None
}
