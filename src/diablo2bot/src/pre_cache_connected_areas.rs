use std::thread;

use crate::{
    bot_settings::BotSettings,
    constants::connected_areas::get_connected_areas,
    enums::{act::Act, game_difficulty::GameDifficulty},
    file_io::FileIo,
    mpq_archives::archives::Archives,
    pattern_matcher_monsters::{cache_monster_tree, MonsterMatcherConfig, Tree},
    string_tables::ZoneNameConverter,
};

pub fn pre_cache_connected_areas(
    level_name: &str,
    act: Act,
    game_difficulty: GameDifficulty,
    bot_settings: &BotSettings,
    file_io: &FileIo,
    archives: &Archives,
    zone_name_converter: &ZoneNameConverter,
) {
    for connected_area in get_connected_areas(level_name) {
        if Tree::has_cache(file_io, connected_area) {
            continue;
        }

        println!("pre-caching \"{}\"", connected_area);

        let monsters_matcher_config = MonsterMatcherConfig::new_levels_monster_matcher_config(
            act,
            game_difficulty,
            bot_settings,
        );

        let mut args = CacheAreaArgs {
            archives: archives.clone(),
            area: connected_area.to_string(),
            file_io: file_io.clone(),
            monsters_matcher_config,
            zone_name_converter: zone_name_converter.clone(),
        };

        if bot_settings.pre_cache_connected_areas_multiple_threads {
            thread::spawn(move || {
                cache_area(&mut args);
            });
        } else {
            cache_area(&mut args);
        }
    }
}

struct CacheAreaArgs {
    archives: Archives,
    file_io: FileIo,
    area: String,
    monsters_matcher_config: MonsterMatcherConfig,
    zone_name_converter: ZoneNameConverter,
}

fn cache_area(args: &mut CacheAreaArgs) {
    cache_monster_tree(
        &mut args.archives,
        &args.file_io,
        &args.area,
        &args.monsters_matcher_config,
        &args.zone_name_converter,
    );
}
