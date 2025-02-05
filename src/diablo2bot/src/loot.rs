use crate::{
    constants::{
        game_window_areas::NOISY_AREAS,
        item_prefixes::ITEM_PREFIXES,
        potion_names::SORTED_POTION_NAMES,
        spells::{TELEKINESIS, TELEPORT},
    },
    enums::{
        click_type::ClickType, game_interface_element::GameInterfaceElement, quality::Quality,
        weapon_set::WeaponSet,
    },
    game::Game,
    game_interface_element_controller::GameInterfaceElementController,
    matrix::Matrix,
    point_u16::PointU16,
    structs::{Item, ItemsFilter},
    table::Table,
    utils::sleep_frames,
    wait_while_moving,
};

struct GoldPile {
    amount: u32,
    point: PointU16,
}

struct FilteredItems {
    gold_piles: Vec<GoldPile>,
    potions: Vec<Item>,
    other_items: Vec<Item>,
}

pub fn pickup_loot(g: &mut Game) {
    let mut filtered_items = get_filtered_items(g);

    let picked_up_gold = pickup_gold_pile(g, &filtered_items);

    if picked_up_gold {
        filtered_items = get_filtered_items(g);
    }

    let picked_up_potion = pickup_potions(g, &filtered_items);

    if picked_up_potion {
        g.output_controller.move_mouse_to_safe_point();

        GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Items).unwrap(); // TODO Throw error

        let img = GameInterfaceElementController::activate_element(g, GameInterfaceElement::Belt)
            .unwrap()
            .unwrap();

        let matrix = img.to_matrix(&g.palette_transformer);

        g.belt.update_belt_and_remove_unneeded_potions(
            &matrix,
            &g.consumable_items_table_matcher,
            &mut g.output_controller,
        );

        GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Belt).unwrap(); // TODO Throw error

        GameInterfaceElementController::activate_element(g, GameInterfaceElement::Items).unwrap();

        sleep_frames(
            g.bot_settings
                .loot_settings
                .num_frames_to_sleep_after_activating_loot_text,
        );

        filtered_items = get_filtered_items(g);
    }

    let picked_up_item = pickup_items(g, &filtered_items);

    if picked_up_gold || picked_up_potion || picked_up_item {
        g.spell_caster
            .activate_skill(TELEPORT, WeaponSet::Primary, &mut g.output_controller);
    }
}

fn remove_prefixes_from_item_name(name: &str) -> String {
    let mut trimmed_name = name.to_string();

    for &prefix in &ITEM_PREFIXES {
        if trimmed_name.starts_with(prefix) {
            trimmed_name = trimmed_name[prefix.len()..].to_string();
        }
    }

    trimmed_name.trim().to_string()
}

fn get_potion_sort_value(name: &str) -> usize {
    for (i, potion_name) in SORTED_POTION_NAMES.iter().enumerate() {
        if name == *potion_name {
            return i;
        }
    }

    SORTED_POTION_NAMES.len()
}

fn filter_items(
    items: &[Item],
    min_gold_to_pickup: u32,
    items_filter: &ItemsFilter,
    inventory: &Table,
) -> FilteredItems {
    let mut gold_piles = Vec::new();
    let mut potions = Vec::new();
    let mut other_items = Vec::new();

    for item in items {
        if let Some(gold_item) = parse_gold_pile(item) {
            if gold_item.amount >= min_gold_to_pickup {
                gold_piles.push(gold_item);
            }
        } else {
            let item_name = remove_prefixes_from_item_name(&item.name);

            match items_filter.items.get(&item_name) {
                Some(item_filter) => {
                    let is_item_wanted = match item.quality {
                        Quality::Grey => item_filter.grey,
                        Quality::Common => item_filter.common,
                        Quality::Magic => item_filter.magic,
                        Quality::Rare => item_filter.rare,
                        Quality::Set => item_filter.set,
                        Quality::Unique => item_filter.unique,
                        Quality::Rune => item_filter.rune,
                    };

                    if is_item_wanted {
                        if is_item_potion(item) {
                            potions.push(item.clone())
                        } else if inventory.has_space_for_item(
                            u32::from(item_filter.row_size),
                            u32::from(item_filter.col_size),
                        ) {
                            other_items.push(item.clone());
                        }
                    }
                }
                None => {
                    println!("Item: {item_name} is not in the filter_items dictionary");
                }
            }
        }
    }

    gold_piles.sort_by(|a, b| b.amount.cmp(&a.amount));
    potions.sort_by_key(|p| get_potion_sort_value(&p.name));

    FilteredItems {
        gold_piles,
        potions,
        other_items,
    }
}

pub fn is_item_potion(item: &Item) -> bool {
    if item.quality != Quality::Common {
        return false;
    }

    for potion_name in SORTED_POTION_NAMES {
        if potion_name == item.name {
            return true;
        }
    }

    false
}

fn parse_gold_pile(item: &Item) -> Option<GoldPile> {
    if item.quality != Quality::Common {
        return None;
    }

    let parts: Vec<&str> = item.name.split_whitespace().collect();

    if parts.len() != 2 {
        return None;
    }

    if parts[1] != "Gold" {
        return None;
    }

    let amount = parts[0].parse().ok()?;

    Some(GoldPile {
        amount,
        point: item.point,
    })
}

pub fn get_inventory_table(g: &mut Game) -> Table {
    g.output_controller.move_mouse_to_safe_point();

    let img = GameInterfaceElementController::activate_element(g, GameInterfaceElement::Inventory)
        .unwrap()
        .unwrap();

    let matrix = img.to_matrix(&g.palette_transformer);

    GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Inventory).unwrap(); // TODO Throw error

    g.table_inventory_empty_matcher.match_from_matrix(&matrix)
}

fn pickup_items(g: &mut Game, initial_filtered_items: &FilteredItems) -> bool {
    let picked_up_item = _pickup_item(g, initial_filtered_items);

    if picked_up_item {
        g.inventory = get_inventory_table(g);
    } else {
        return false;
    }

    for _ in 0..g
        .bot_settings
        .loot_settings
        .max_items_to_pickup_per_loot_session
    {
        let filtered_items = get_filtered_items(g);

        let _picked_up_item = _pickup_item(g, &filtered_items);

        if _picked_up_item {
            g.inventory = get_inventory_table(g);
        } else {
            break;
        }
    }

    picked_up_item
}

fn _pickup_item(g: &mut Game, filtered_items: &FilteredItems) -> bool {
    match filtered_items.other_items.first() {
        Some(item) => {
            pickup_item(g, item.point, false);
            g.output_controller.move_mouse_to_safe_point();
            sleep_frames(
                g.bot_settings
                    .loot_settings
                    .num_frames_to_sleep_after_picking_up_item,
            );

            true
        }
        None => false,
    }
}

fn pickup_potions(g: &mut Game, initial_filtered_items: &FilteredItems) -> bool {
    let picked_up_item = _pickup_potion(g, initial_filtered_items);

    if !picked_up_item {
        return false;
    }

    for _ in 0..g
        .bot_settings
        .loot_settings
        .max_potions_to_pickup_per_loot_session
    {
        let filtered_items = get_filtered_items(g);

        let _picked_up_item = _pickup_potion(g, &filtered_items);

        if !_picked_up_item {
            break;
        }
    }

    picked_up_item
}

fn _pickup_potion(g: &mut Game, filtered_items: &FilteredItems) -> bool {
    for item in &filtered_items.potions {
        let has_space_for_potion = g.belt.auto_add_item(&item.name);

        if has_space_for_potion {
            pickup_item(g, item.point, true);

            g.output_controller.move_mouse_to_safe_point();
            sleep_frames(
                g.bot_settings
                    .loot_settings
                    .num_frames_to_sleep_after_picking_up_item,
            );

            return true;
        }
    }

    false
}

fn pickup_gold_pile(g: &mut Game, initial_filtered_items: &FilteredItems) -> bool {
    let picked_up_item = _pickup_gold_pile(g, initial_filtered_items);

    if !picked_up_item {
        return false;
    }

    for _ in 0..g
        .bot_settings
        .loot_settings
        .max_gold_piles_to_pickup_per_loot_session
    {
        let filtered_items = get_filtered_items(g);

        let _picked_up_item = _pickup_gold_pile(g, &filtered_items);

        if !_picked_up_item {
            break;
        }
    }

    picked_up_item
}

fn _pickup_gold_pile(g: &mut Game, filtered_items: &FilteredItems) -> bool {
    match filtered_items.gold_piles.first() {
        Some(highest_gold_amount_item) => {
            pickup_item(g, highest_gold_amount_item.point, true);
            g.output_controller.move_mouse_to_safe_point();
            sleep_frames(
                g.bot_settings
                    .loot_settings
                    .num_frames_to_sleep_after_picking_up_item,
            );

            true
        }
        None => false,
    }
}

fn pickup_item(g: &mut Game, point: PointU16, can_be_picked_up_with_telekinesis: bool) {
    // The point is the top left corner of the item text. We want to click inside the item text
    let item_point = PointU16::new(point.row + 5, point.col + 5);

    match can_be_picked_up_with_telekinesis {
        true => match g.spell_caster.has_skill(TELEKINESIS) {
            true => {
                g.spell_caster.use_skill(
                    TELEKINESIS,
                    item_point,
                    true,
                    WeaponSet::Primary,
                    &mut g.output_controller,
                );
            }
            false => {
                g.output_controller
                    .click_mouse(item_point, ClickType::Left, true, true);
            }
        },
        false => {
            g.output_controller
                .click_mouse(item_point, ClickType::Left, true, true);
            wait_while_moving(g)
        }
    }
}

fn get_filtered_items(g: &mut Game) -> FilteredItems {
    let matrix = take_loot_screenshot(g);
    let items = g.font_symbol_matcher.match_image_items(&matrix);

    filter_items(
        &items,
        g.profile.min_gold_to_pickup,
        &g.item_filter,
        &g.inventory,
    )
}

pub fn take_loot_screenshot(g: &mut Game) -> Matrix {
    let mut matrix = g
        .game_screenshotter
        .take_screenshot()
        .to_matrix(&g.palette_transformer);

    matrix.clear_areas(&NOISY_AREAS);

    matrix
}
