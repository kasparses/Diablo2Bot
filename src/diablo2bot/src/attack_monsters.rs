use crate::{
    constants::{
        game_window_areas::NOISY_AREAS_KEEP_HEALTH_MANA_TEXT,
        game_window_points::MIDDLE_POINT,
        misc::MAX_ITEM_SIZE,
        spells::{STATIC_FIELD, TELEPORT},
    },
    enums::{
        errors::{BotError, CharacterHasDiedError, LowInventorySpaceError},
        game_interface_element::GameInterfaceElement,
        state::State,
        weapon_set::WeaponSet,
    },
    game::Game,
    game_interface_element_controller::GameInterfaceElementController,
    health_mana::{
        check_health_hard_limit, drink_potions_if_points_under_soft_limit, get_health, get_mana,
    },
    image::Image,
    loot::pickup_loot,
    matrix::Matrix,
    point_u16::PointU16,
    state_validator::is_in_enum_state,
    utils::sleep_frames,
};

pub fn attack_monsters_and_loot(g: &mut Game) -> Result<(), BotError> {
    let attacked_monsters = attack_monsters(g)?;

    if attacked_monsters {
        g.output_controller.move_mouse_to_safe_point();

        if g.spell_caster.has_skill(TELEPORT) {
            g.spell_caster
                .activate_skill(TELEPORT, WeaponSet::Primary, &mut g.output_controller);
        }

        GameInterfaceElementController::activate_element(g, GameInterfaceElement::Items)?;

        g.output_controller.move_mouse_to_safe_point();
        sleep_frames(
            g.bot_settings
                .num_frames_to_sleep_before_looting_after_attacking_monsters,
        );

        pickup_loot(g);

        GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Items)?;

        if !g.inventory.has_empty_cell_area(MAX_ITEM_SIZE) {
            return Err(BotError::LowInventorySpace(LowInventorySpaceError));
        }
    }

    Ok(())
}

fn attack_monsters(g: &mut Game) -> Result<bool, BotError> {
    let mut monster_points = find_monsters(g)?;

    if monster_points.is_empty() {
        return Ok(false);
    }

    GameInterfaceElementController::deactivate_element(g, GameInterfaceElement::Automap)?;

    let cooldown = g
        .spell_caster
        .get_spell_cooldown(&g.profile.primary_attack_skill, WeaponSet::Primary);

    if g.spell_caster.has_skill(STATIC_FIELD) && monster_points.len() > 4 {
        for _ in 0..2 {
            g.spell_caster.use_skill(
                STATIC_FIELD,
                MIDDLE_POINT,
                false,
                WeaponSet::Primary,
                &mut g.output_controller,
            );
        }
    }

    let mut in_combat_count = 0;
    let mut sequential_no_monsters_count = 0;

    loop {
        for (i, point) in monster_points.iter().enumerate() {
            g.spell_caster.use_skill(
                &g.profile.primary_attack_skill,
                *point,
                false,
                WeaponSet::Primary,
                &mut g.output_controller,
            );

            sleep_frames(cooldown);

            if i == 2 {
                break;
            }
        }

        monster_points = find_monsters(g)?;

        if in_combat_count >= 4 || sequential_no_monsters_count >= 1 {
            break;
        }

        if monster_points.is_empty() {
            sequential_no_monsters_count += 1;
            sleep_frames(g.bot_settings.num_frames_to_sleep_after_attacking_monsters);
        } else {
            sequential_no_monsters_count = 0;
        }

        in_combat_count += 1;
    }

    Ok(true)
}

fn find_monsters(g: &mut Game) -> Result<Vec<PointU16>, BotError> {
    let matrix = take_monster_screenshot(g)?;

    let matches = g.monster_matcher.look_up(&matrix);

    let mut match_points = Vec::new();

    if matches.is_empty() {
        return Ok(match_points);
    }

    sleep_frames(
        g.bot_settings
            .num_frames_to_sleep_after_scanning_screen_for_monsters,
    );

    let matrix2 = take_monster_screenshot(g)?;

    for m in &matches {
        let window_1 = matrix.get_window(m.window_offset_point);
        let window_2 = matrix2.get_window(m.window_offset_point);

        if window_1 != window_2 {
            match_points.push(m.window_offset_point);
        }
    }

    Ok(match_points)
}

fn take_monster_screenshot(g: &mut Game) -> Result<Matrix, BotError> {
    g.output_controller.move_mouse_to_safe_point();

    let img = g.game_screenshotter.take_screenshot();

    check_if_we_have_died(&img)?;

    let mut matrix = img.to_matrix(&g.palette_transformer);

    matrix.clear_areas(&NOISY_AREAS_KEEP_HEALTH_MANA_TEXT);

    let health_points = get_health(&matrix, &g.font_symbol_matcher);
    let mana_points = get_mana(&matrix, &g.font_symbol_matcher);

    if let Some(health_points) = health_points {
        check_health_hard_limit(health_points, g.profile.health_limit_hard)?;
    }

    drink_potions_if_points_under_soft_limit(g, health_points, mana_points)?;

    Ok(matrix)
}

fn check_if_we_have_died(img: &Image) -> Result<(), BotError> {
    match is_in_enum_state(State::HasDied, img) {
        true => Err(BotError::CharacterHasDied(CharacterHasDiedError)),
        false => Ok(()),
    }
}
