use crate::{
    bot_settings::BotSettings,
    constants::{game_window_points::MIDDLE_POINT, spells::TELEPORT},
    enums::weapon_set::WeaponSet,
    game::Game,
    output_controller::OutputController,
    spell_caster::SpellCaster,
    utils::sleep_frames,
    weapon_swapper::WeaponSwapper,
};

pub struct Buffs {
    bot_settings: BotSettings,
    buffs_only_castable_on_primary_weaponset: Vec<Buff>,
    buffs_only_castable_on_secondary_weaponset: Vec<Buff>,
}

struct Buff {
    skill: String,
    duration_seconds: u32,
    last_updated_time_seconds: Option<u64>,
}

impl Buff {
    fn is_expired(&self, current_time_seconds: u64) -> bool {
        match self.last_updated_time_seconds {
            Some(last_updated_time_seconds) => {
                let time_since_last_update = current_time_seconds - last_updated_time_seconds;
                time_since_last_update > u64::from(self.duration_seconds)
            }
            None => true,
        }
    }

    fn cast(
        &mut self,
        active_weaponset: WeaponSet,
        output_controller: &mut OutputController,
        spell_caster: &mut SpellCaster,
    ) {
        spell_caster.use_skill(
            &self.skill,
            MIDDLE_POINT,
            true,
            active_weaponset,
            output_controller,
        );

        self.last_updated_time_seconds = Some(get_current_time_seconds());
    }
}

impl Buffs {
    pub fn new(buffs: &[crate::profile::Buff], bot_settings: BotSettings) -> Self {
        let mut buffs_only_castable_on_primary_weaponset = Vec::new();
        let mut buffs_only_castable_on_secondary_weaponset = Vec::new();

        for buff in buffs {
            let b = Buff {
                skill: buff.skill.clone(),
                duration_seconds: buff.duration,
                last_updated_time_seconds: None,
            };

            if buff.only_castable_on_secondary_weaponset {
                buffs_only_castable_on_secondary_weaponset.push(b);
            } else {
                buffs_only_castable_on_primary_weaponset.push(b);
            }
        }

        Self {
            buffs_only_castable_on_primary_weaponset,
            buffs_only_castable_on_secondary_weaponset,
            bot_settings,
        }
    }

    pub fn reset(&mut self) {
        for buff in &mut self.buffs_only_castable_on_primary_weaponset {
            buff.last_updated_time_seconds = None;
        }

        for buff in &mut self.buffs_only_castable_on_secondary_weaponset {
            buff.last_updated_time_seconds = None;
        }
    }

    fn has_expired_buffs_only_castable_on_secondary_weaponset(&self, current_time: u64) -> bool {
        self.buffs_only_castable_on_secondary_weaponset
            .iter()
            .any(|buff| buff.is_expired(current_time))
    }
}

fn get_current_time_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn re_cast_buffs_if_expired(g: &mut Game) {
    let current_time = get_current_time_seconds();

    let mut recast_buff = false;

    if g.buffs
        .has_expired_buffs_only_castable_on_secondary_weaponset(current_time)
    {
        WeaponSwapper::switch_to_weaponset(g, WeaponSet::Secondary);

        for buff in &mut g.buffs.buffs_only_castable_on_secondary_weaponset {
            if buff.is_expired(current_time) {
                buff.cast(
                    g.weapon_swapper.current_weaponset,
                    &mut g.output_controller,
                    &mut g.spell_caster,
                );
                recast_buff = true;
            }
        }

        sleep_frames(
            g.buffs
                .bot_settings
                .num_frames_to_sleep_after_casting_buffs_on_secondary_weaponset,
        );
    }

    WeaponSwapper::switch_to_weaponset(g, WeaponSet::Primary);

    for buff in &mut g.buffs.buffs_only_castable_on_primary_weaponset {
        if buff.is_expired(current_time) {
            buff.cast(
                g.weapon_swapper.current_weaponset,
                &mut g.output_controller,
                &mut g.spell_caster,
            );
            recast_buff = true;
        }
    }

    if recast_buff {
        g.spell_caster.activate_skill(
            TELEPORT,
            g.weapon_swapper.current_weaponset,
            &mut g.output_controller,
        );
        g.spell_caster.sleep_until_ready();
    }
}
