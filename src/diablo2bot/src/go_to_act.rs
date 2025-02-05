use crate::{
    _walk_to_waypoint,
    constants::game_window_points::TOWN_ZONE_WAYPOINT_MENU_POINT,
    enums::{act::Act, state::State},
    game::Game,
    state_validator::wait_while_in_enum_state,
};

pub fn go_to_act(g: &mut Game, current_act: Act, target_act: Act) {
    _walk_to_waypoint(g, current_act);

    g.output_controller.click_mouse(
        target_act.to_waypoint_act_tab(),
        crate::enums::click_type::ClickType::Left,
        true,
        true,
    );

    g.output_controller.click_mouse(
        TOWN_ZONE_WAYPOINT_MENU_POINT,
        crate::enums::click_type::ClickType::Left,
        true,
        true,
    );

    g.output_controller.move_mouse_to_safe_point();

    wait_while_in_enum_state(
        g,
        State::WaypointMenu,
        g.bot_settings.max_frames_to_wait_for_zone_load,
    )
    .expect("Could not enter zone from waypoint!");
}
