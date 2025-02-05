use std::collections::HashMap;

use crate::{
    constants::game_window_areas::LEFT_SKILL_ICON_AREA,
    enums::weapon_set::WeaponSet,
    game::Game,
    matrix::Matrix,
    output_controller::map_key_code,
    point_u16::PointU16,
    skill_icon_getter::SkillIconGetter,
    units::Frames,
    utils::{sleep_frame, sleep_frames},
};

pub struct WeaponSwapper {
    pub current_weaponset: WeaponSet,
    left_skill_icon_weaponset_primary: Matrix,
    left_skill_icon_weaponset_secondary: Matrix,
    swap_weapons_keybinding: enigo::Key,
}

impl WeaponSwapper {
    pub fn new(
        miscellaneous_keybindings: &HashMap<String, String>,
        skill_icon_getter: &SkillIconGetter,
        left_skill_weaponset_primary: &str,
        left_skill_weaponset_secondary: &str,
    ) -> Self {
        let left_skill_icon_weaponset_primary = skill_icon_getter
            .get_skill_icon_sprite(left_skill_weaponset_primary)
            .clone();
        let left_skill_icon_weaponset_secondary = skill_icon_getter
            .get_skill_icon_sprite(left_skill_weaponset_secondary)
            .clone();

        let swap_weapons_keybinding =
            map_key_code(miscellaneous_keybindings.get("swap_weapons").unwrap());

        Self {
            current_weaponset: WeaponSet::Primary,
            left_skill_icon_weaponset_primary,
            left_skill_icon_weaponset_secondary,
            swap_weapons_keybinding,
        }
    }

    pub fn switch_to_weaponset(g: &mut Game, target_weaponset: WeaponSet) {
        if g.weapon_swapper.current_weaponset == target_weaponset {
            return;
        }

        // We cannot swap weapons while in spell cooldown.
        g.spell_caster.sleep_until_ready();

        for i in 0..100 {
            if i % 10 == 0 {
                g.output_controller
                    .click_key(g.weapon_swapper.swap_weapons_keybinding);
            }

            if Self::has_target_weaponset_skill_icon(g, target_weaponset) {
                sleep_frames(Frames(4));

                // Checking again to prevent race conditions
                if Self::has_target_weaponset_skill_icon(g, target_weaponset) {
                    g.weapon_swapper.current_weaponset = target_weaponset;
                    return;
                }
            }

            sleep_frame();
        }

        panic!("Could not swap weapons!");
    }

    fn get_target_weaponset_left_skill_icon(&self, weaponset: WeaponSet) -> &Matrix {
        match weaponset {
            WeaponSet::Primary => &self.left_skill_icon_weaponset_primary,
            WeaponSet::Secondary => &self.left_skill_icon_weaponset_secondary,
        }
    }

    fn has_target_weaponset_skill_icon(g: &mut Game, target_weaponset: WeaponSet) -> bool {
        let matrix = g
            .game_screenshotter
            .take_screenshot()
            .to_matrix(&g.palette_transformer);

        let mut left_skill_icon = matrix.get_sub_matrix2(LEFT_SKILL_ICON_AREA);
        left_skill_icon.set_value(PointU16::new(0, 0), 0);

        left_skill_icon.data
            == g.weapon_swapper
                .get_target_weaponset_left_skill_icon(target_weaponset)
                .data
    }
}
