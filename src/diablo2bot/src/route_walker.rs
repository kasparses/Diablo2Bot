use crate::{
    constants::{
        routes::{
            ACT_1_START_TO_DECKARD_CAIN_POINTS, ACT_1_START_TO_POTION_SELLER_POINTS,
            ACT_1_START_TO_STASH_POINTS, ACT_1_START_TO_WAYPOINT_1_POINTS,
            ACT_1_START_TO_WAYPOINT_2_POINTS, ACT_1_START_TO_WAYPOINT_3_POINTS,
            ACT_1_START_TO_WAYPOINT_4_POINTS, ACT_2_START_TO_DECKARD_CAIN_POINTS,
            ACT_2_START_TO_POTION_SELLER_POINTS, ACT_2_START_TO_STASH_POINTS,
            ACT_2_START_TO_WAYPOINT_POINTS, ACT_3_START_TO_DECKARD_CAIN_POINTS,
            ACT_3_START_TO_POTION_SELLER_POINTS, ACT_3_START_TO_STASH_POINTS,
            ACT_3_START_TO_WAYPOINT_POINTS, ACT_4_START_TO_DECKARD_CAIN_POINTS,
            ACT_4_START_TO_POTION_SELLER_POINTS, ACT_4_START_TO_STASH_POINTS,
            ACT_4_START_TO_WAYPOINT_POINTS, ACT_5_START_TO_DECKARD_CAIN_POINTS,
            ACT_5_START_TO_POTION_SELLER_POINTS, ACT_5_START_TO_STASH_POINTS,
            ACT_5_START_TO_WAYPOINT_POINTS,
        },
        spells::TELEKINESIS,
    },
    enums::{
        click_type::ClickType, errors::WrongGameStateError, route::Route, state::State,
        weapon_set::WeaponSet,
    },
    game::Game,
    image::Image,
    point_u16::PointU16,
    state_validator::wait_for_enum_state,
    wait_while_moving,
};

pub fn get_route_points(route: Route) -> &'static [PointU16] {
    match route {
        Route::Act1StartToPotionSeller => &ACT_1_START_TO_POTION_SELLER_POINTS,
        Route::Act2StartToPotionSeller => &ACT_2_START_TO_POTION_SELLER_POINTS,
        Route::Act3StartToPotionSeller => &ACT_3_START_TO_POTION_SELLER_POINTS,
        Route::Act4StartToPotionSeller => &ACT_4_START_TO_POTION_SELLER_POINTS,
        Route::Act5StartToPotionSeller => &ACT_5_START_TO_POTION_SELLER_POINTS,

        Route::Act1StartToStash => &ACT_1_START_TO_STASH_POINTS,
        Route::Act2StartToStash => &ACT_2_START_TO_STASH_POINTS,
        Route::Act3StartToStash => &ACT_3_START_TO_STASH_POINTS,
        Route::Act4StartToStash => &ACT_4_START_TO_STASH_POINTS,
        Route::Act5StartToStash => &ACT_5_START_TO_STASH_POINTS,

        Route::Act1StartToDeckardCain => &ACT_1_START_TO_DECKARD_CAIN_POINTS,
        Route::Act2StartToDeckardCain => &ACT_2_START_TO_DECKARD_CAIN_POINTS,
        Route::Act3StartToDeckardCain => &ACT_3_START_TO_DECKARD_CAIN_POINTS,
        Route::Act4StartToDeckardCain => &ACT_4_START_TO_DECKARD_CAIN_POINTS,
        Route::Act5StartToDeckardCain => &ACT_5_START_TO_DECKARD_CAIN_POINTS,

        Route::Act1StartToWaypoint1 => &ACT_1_START_TO_WAYPOINT_1_POINTS,
        Route::Act1StartToWaypoint2 => &ACT_1_START_TO_WAYPOINT_2_POINTS,
        Route::Act1StartToWaypoint3 => &ACT_1_START_TO_WAYPOINT_3_POINTS,
        Route::Act1StartToWaypoint4 => &ACT_1_START_TO_WAYPOINT_4_POINTS,
        Route::Act2StartToWaypoint => &ACT_2_START_TO_WAYPOINT_POINTS,
        Route::Act3StartToWaypoint => &ACT_3_START_TO_WAYPOINT_POINTS,
        Route::Act4StartToWaypoint => &ACT_4_START_TO_WAYPOINT_POINTS,
        Route::Act5StartToWaypoint => &ACT_5_START_TO_WAYPOINT_POINTS,
        // Route::Act1StartToCharsi => &ACT_1_START_TO_CHARSI_POINTS,
        // Route::Act2StartToFara => &ACT_2_START_TO_FARA_POINTS,
        // Route::Act3StartToHratli => &ACT_3_START_TO_HRATLI_POINTS,
        // Route::Act4StartToHalbu => &ACT_4_START_TO_HALBU_POINTS,
        // Route::Act5StartToLarzuk => &ACT_5_START_TO_LARZUK_POINTS,
    }
}

pub fn is_last_element(i: usize, len: usize) -> bool {
    len == 0 || i == len - 1
}

pub fn walk_enum_route_with_end_state(
    g: &mut Game,
    route: Route,
    end_state: State,
) -> Result<Image, WrongGameStateError> {
    let route_points = get_route_points(route);

    if g.spell_caster.has_skill(TELEKINESIS) {
        g.spell_caster
            .activate_skill(TELEKINESIS, WeaponSet::Primary, &mut g.output_controller);
    }

    for (i, point) in route_points.iter().enumerate() {
        if is_last_element(i, route_points.len()) {
            if g.spell_caster.has_skill(TELEKINESIS) {
                g.spell_caster.use_skill(
                    TELEKINESIS,
                    *point,
                    true,
                    WeaponSet::Primary,
                    &mut g.output_controller,
                );
            } else {
                g.output_controller
                    .click_mouse(*point, ClickType::Left, true, true);
            }
        } else {
            g.output_controller
                .click_mouse(*point, ClickType::Left, true, true);
        }

        wait_while_moving(g);
    }

    let img = wait_for_enum_state(g, end_state, 100)?;
    Ok(img)
}

pub fn walk_enum_route_without_end_state(g: &mut Game, route: Route) {
    let route_points = get_route_points(route);

    for point in route_points {
        g.output_controller
            .click_mouse(*point, ClickType::Left, true, true);

        wait_while_moving(g);
    }
}
