use crate::{
    buffs::re_cast_buffs_if_expired,
    change_level, do_first_run_actions,
    enums::{errors::BotError, game_interface_element::GameInterfaceElement::Automap},
    game::Game,
    game_interface_element_controller::GameInterfaceElementController,
    get_path_and_walk_it, read_current_zone, take_screenshot, walk_to_waypoint, waypoint_to_zone,
    zone_traveller::ZoneTraveller,
};

pub fn run(g: &mut Game, is_first_run: bool) -> Result<(), BotError> {
    walk_to_waypoint(g);

    waypoint_to_zone(g);

    if is_first_run {
        do_first_run_actions(g)?;
    }

    GameInterfaceElementController::activate_element(g, Automap).unwrap();

    let matrix = take_screenshot(g);

    g.buffs.reset();

    let mut zone_traveller = ZoneTraveller::new(
        g.bot_settings.movement_settings,
        &g.map_sprite_matcher,
        matrix,
    );

    for i in 0..g
        .bot_settings
        .movement_settings
        .max_automap_path_refresh_before_game_refresh
    {
        println!(
            "{}..{}",
            i + 1,
            g.bot_settings
                .movement_settings
                .max_automap_path_refresh_before_game_refresh
        );

        re_cast_buffs_if_expired(g);

        get_path_and_walk_it(g, &mut zone_traveller)?;

        GameInterfaceElementController::activate_element(g, Automap).unwrap();

        let matrix = take_screenshot(g);

        if let Some(current_zone) = read_current_zone(g, &matrix) {
            change_level(g, &current_zone)?;
        }

        if zone_traveller
            .update_map(
                &g.map_sprite_matcher,
                &matrix,
                &g.pixel_palette,
                &mut g.logger,
            )
            .is_err()
        {
            println!("Could not connect last known location to current location. Resetting map");
            zone_traveller = ZoneTraveller::new(
                g.bot_settings.movement_settings,
                &g.map_sprite_matcher,
                matrix,
            );
        }
    }

    Ok(())
}
