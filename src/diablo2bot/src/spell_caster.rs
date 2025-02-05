use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    constants::spells::{CHAIN_LIGHTNING, LIGHTNING},
    enums::{character_class::CharacterClass, click_type::ClickType, weapon_set::WeaponSet},
    output_controller::{self, map_key_code},
    point_u16::PointU16,
    units::{Frames, Milliseconds},
    utils::{sleep_frame, sleep_frames, sleep_millis},
    weaponset_data::WeaponSetData,
};

struct SpellCasterState {
    last_skill_used_cooldown: Frames,
    last_skill_used_time: Milliseconds,
}

#[derive(Debug)]
struct SpellData {
    cast_time: WeaponSetData<Frames>,
    cooldown: WeaponSetData<Frames>,
    cooldown_lightning: WeaponSetData<Frames>,
}

impl SpellData {
    fn new(character_class: CharacterClass, faster_cast_rate: &WeaponSetData<u32>) -> Self {
        let cast_milliseconds = WeaponSetData::new(
            Self::calculate_cast_frames(character_class, faster_cast_rate.get(WeaponSet::Primary)),
            Self::calculate_cast_frames(
                character_class,
                faster_cast_rate.get(WeaponSet::Secondary),
            ),
        );

        let cooldown_milliseconds = WeaponSetData::new(
            Self::calculate_cooldown_frames(
                character_class,
                faster_cast_rate.get(WeaponSet::Primary),
                false,
            ),
            Self::calculate_cooldown_frames(
                character_class,
                faster_cast_rate.get(WeaponSet::Secondary),
                false,
            ),
        );

        let cooldown_milliseconds_lightning = WeaponSetData::new(
            Self::calculate_cooldown_frames(
                character_class,
                faster_cast_rate.get(WeaponSet::Primary),
                true,
            ),
            Self::calculate_cooldown_frames(
                character_class,
                faster_cast_rate.get(WeaponSet::Secondary),
                true,
            ),
        );

        Self {
            cast_time: cast_milliseconds,
            cooldown: cooldown_milliseconds,
            cooldown_lightning: cooldown_milliseconds_lightning,
        }
    }

    fn get_spell_cast_time(&self, active_weaponset: WeaponSet) -> Frames {
        self.cast_time.get(active_weaponset)
    }

    fn get_spell_cooldown(&self, skill_name: &str, active_weaponset: WeaponSet) -> Frames {
        if [LIGHTNING, CHAIN_LIGHTNING].contains(&skill_name) {
            self.cooldown_lightning.get(active_weaponset)
        } else {
            self.cooldown.get(active_weaponset)
        }
    }

    /*
    Calculate the number of frames it takes before the spell is cast.
    Example of casting teleport with on a Sorceress with 120 faster_cast_rate:
    Frame number
    0               click mouse
    1               spell animation
    2               spell animation
    3               spell animation
    4               spell animation
    5               spell is cast!          This is the frame number that this function returns
    6               finishes animation
    7               finishes animation
    8               finishes animation

    Args:
        faster_cast_rate (int): The amount of faster_cast_rate we have from our gear
    */
    fn calculate_cast_frames(character_class: CharacterClass, faster_cast_rate: u32) -> Frames {
        let effective_faster_cast_rate =
            Self::calculate_effective_faster_cast_rate(faster_cast_rate);
        let animation_speed = Self::get_animation_speed(character_class);
        let base_action_flag = Self::get_base_action_flag(character_class);

        Frames(
            (base_action_flag as f32 * 256.0
                / (animation_speed as f32 * (100.0 + effective_faster_cast_rate as f32) / 100.0)
                    .floor())
            .ceil() as u64,
        )
    }

    /*
    This function calculates the total amount of frames it takes before we can cast a new spell after casting a spell
    Example of casting teleport with a Sorceress with 120 faster_cast_rate:
    Frame number
    0               click mouse
    1               spell animation
    2               spell animation
    3               spell animation
    4               spell animation
    5               spell is cast!
    6               finishes animation
    7               finishes animation
    8               finishes animation      This is the frame number that this function returns
    9               ready to cast new spell

    Args:
        faster_cast_rate (int): The amount of faster_cast_rate we have from our gear
    */
    fn calculate_cooldown_frames(
        character_class: CharacterClass,
        faster_cast_rate: u32,
        calculate_for_lightning_skill: bool,
    ) -> Frames {
        let effective_faster_cast_rate =
            Self::calculate_effective_faster_cast_rate(faster_cast_rate);
        let animation_speed = Self::get_animation_speed(character_class);
        let casting_base = Self::get_casting_base(character_class, calculate_for_lightning_skill);

        match calculate_for_lightning_skill {
            true => Frames(
                (casting_base as f32 * 256.0
                    / (animation_speed as f32 * (100.0 + effective_faster_cast_rate as f32)
                        / 100.0)
                        .floor())
                .ceil() as u64,
            ),
            false => Frames(
                (casting_base as f32 * 256.0
                    / (animation_speed as f32 * (100.0 + effective_faster_cast_rate as f32)
                        / 100.0)
                        .floor()
                    - 1.0)
                    .ceil() as u64,
            ),
        }
    }

    fn calculate_effective_faster_cast_rate(faster_cast_rate: u32) -> u32 {
        // https://d2.maxroll.gg/resources/breakpoints-animations
        (faster_cast_rate * 120) / (faster_cast_rate + 120)
    }

    fn get_casting_base(
        character_class: CharacterClass,
        calculate_for_lightning_skill: bool,
    ) -> u32 {
        match calculate_for_lightning_skill {
            true => match character_class {
                CharacterClass::Amazon => 20,
                CharacterClass::Assassin => 17,
                CharacterClass::Sorceress => 19,
                CharacterClass::Barbarian => 14,
                CharacterClass::Druid | CharacterClass::Paladin => 16,
                CharacterClass::Necromancer => 15,
            },
            false => match character_class {
                CharacterClass::Amazon => 20,
                CharacterClass::Assassin => 17,
                CharacterClass::Sorceress | CharacterClass::Barbarian => 14,
                CharacterClass::Druid | CharacterClass::Paladin => 16,
                CharacterClass::Necromancer => 15,
            },
        }
    }

    fn get_animation_speed(character_class: CharacterClass) -> u32 {
        match character_class {
            CharacterClass::Amazon
            | CharacterClass::Assassin
            | CharacterClass::Barbarian
            | CharacterClass::Necromancer
            | CharacterClass::Sorceress
            | CharacterClass::Paladin => 256,
            CharacterClass::Druid => 208,
        }
    }

    fn get_base_action_flag(character_class: CharacterClass) -> u32 {
        match character_class {
            CharacterClass::Amazon => 13,
            CharacterClass::Assassin | CharacterClass::Barbarian | CharacterClass::Paladin => 9,
            CharacterClass::Druid => 10,
            CharacterClass::Necromancer => 8,
            CharacterClass::Sorceress => 7,
        }
    }
}

pub struct SpellCaster {
    state: Option<SpellCasterState>,
    skill_name_to_keybinding: HashMap<String, enigo::Key>,
    active_skill_primary_weaponset: Option<String>,
    active_skill_secondary_weaponset: Option<String>,
    spell_data: SpellData,
}

impl SpellCaster {
    pub fn new(
        skill_keybindings: &HashMap<String, String>,
        character_class: CharacterClass,
        faster_cast_rate: &WeaponSetData<u32>,
    ) -> Self {
        let mut skill_name_to_keybinding = HashMap::new();

        for (skill_name, keybinding) in skill_keybindings {
            let key = map_key_code(keybinding);
            skill_name_to_keybinding.insert(skill_name.clone(), key);
        }

        let spell_data = SpellData::new(character_class, faster_cast_rate);

        Self {
            state: None,
            skill_name_to_keybinding,
            active_skill_primary_weaponset: None,
            active_skill_secondary_weaponset: None,
            spell_data,
        }
    }

    pub fn has_skill(&self, skill: &str) -> bool {
        self.skill_name_to_keybinding.contains_key(skill)
    }

    pub fn sleep_cast_time(&self, active_weaponset: WeaponSet) {
        sleep_frames(self.spell_data.get_spell_cast_time(active_weaponset));
    }

    pub fn use_skill(
        &mut self,
        skill_name: &str,
        point: PointU16,
        sleep_after_cursor_movement: bool,
        active_weaponset: WeaponSet,
        output_controller: &mut output_controller::OutputController,
    ) {
        if let Some(key) = self.skill_name_to_keybinding.get(skill_name) {
            output_controller.move_mouse(point);

            if sleep_after_cursor_movement {
                sleep_frame();
            } else {
                sleep_millis(Milliseconds(10));
            }

            self._activate_skill(skill_name, *key, active_weaponset, output_controller);

            self.sleep_until_ready();

            output_controller.click_mouse(point, ClickType::Right, false, false);

            self.state = Some(SpellCasterState {
                last_skill_used_cooldown: self
                    .spell_data
                    .get_spell_cooldown(skill_name, active_weaponset),
                last_skill_used_time: Milliseconds(get_current_time_milliseconds()),
            });
        }
    }

    pub fn get_spell_cooldown(&self, skill_name: &str, active_weaponset: WeaponSet) -> Frames {
        self.spell_data
            .get_spell_cooldown(skill_name, active_weaponset)
    }

    pub fn sleep_until_ready(&self) {
        if let Some(ref state) = self.state {
            let now = Milliseconds(get_current_time_milliseconds());

            let cooldown_end_time =
                state.last_skill_used_time + Milliseconds::from(state.last_skill_used_cooldown);

            if now < cooldown_end_time {
                let remaining_cooldown = cooldown_end_time - now;
                sleep_millis(remaining_cooldown);
            }
        }
    }

    pub fn activate_skill(
        &mut self,
        skill_name: &str,
        active_weaponset: WeaponSet,
        output_controller: &mut output_controller::OutputController,
    ) {
        if let Some(key) = self.skill_name_to_keybinding.get(skill_name) {
            self._activate_skill(skill_name, *key, active_weaponset, output_controller);
        }
    }

    fn _activate_skill(
        &mut self,
        skill_name: &str,
        skill_keybinding: enigo::Key,
        active_weaponset: WeaponSet,
        output_controller: &mut output_controller::OutputController,
    ) {
        match active_weaponset {
            WeaponSet::Primary => match &self.active_skill_primary_weaponset {
                Some(active_skill_primary_weaponset) => {
                    if active_skill_primary_weaponset != skill_name {
                        output_controller.click_key(skill_keybinding);
                        self.active_skill_primary_weaponset = Some(skill_name.to_string());
                        sleep_frame();
                    }
                }
                _ => {
                    output_controller.click_key(skill_keybinding);
                    self.active_skill_primary_weaponset = Some(skill_name.to_string());
                    sleep_frame();
                }
            },
            WeaponSet::Secondary => match &self.active_skill_secondary_weaponset {
                Some(active_skill_secondary_weaponset) => {
                    if active_skill_secondary_weaponset != skill_name {
                        output_controller.click_key(skill_keybinding);
                        self.active_skill_secondary_weaponset = Some(skill_name.to_string());
                        sleep_frame();
                    }
                }
                _ => {
                    output_controller.click_key(skill_keybinding);
                    self.active_skill_secondary_weaponset = Some(skill_name.to_string());
                    sleep_frame();
                }
            },
        }
    }
}

pub fn get_current_time_milliseconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}
