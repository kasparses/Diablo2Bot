use std::io;

use crate::{
    belt::Belt,
    bot_settings::BotSettings,
    buffs::Buffs,
    constants::table_meta_data::INVENTORY_TABLE_META_DATA,
    enums::table_type::TableType,
    file_io::FileIo,
    font_char_map::get_non_control_ascii_char_font_map,
    font_matcher::FontMatcher,
    game_interface_element_controller::GameInterfaceElementController,
    game_screenshotter::GameScreenshotter,
    inventory::TableEmptyMatcher,
    logger::Logger,
    map_matcher::MapMatcher,
    mpq_archives::archives::Archives,
    output_controller::OutputController,
    pal_pl2::{PaletteTransformer, PixelPalette},
    pattern_matcher_monsters::{get_monster_tree, MonsterMatcherConfig, Tree},
    pre_cache_connected_areas::pre_cache_connected_areas,
    profile::Profile,
    skill_icon_getter::SkillIconGetter,
    spell_caster::SpellCaster,
    string_tables::{StringTables, ZoneNameConverter},
    structs::ItemsFilter,
    table::Table,
    table_matcher::ConsumableItemsTableMatcher,
    weapon_swapper::WeaponSwapper,
    weaponset_data::WeaponSetData,
    zone_to_area,
};

pub struct Game {
    pub archives: Archives,
    pub file_io: FileIo,
    pub font_symbol_matcher: FontMatcher,
    pub table_inventory_empty_matcher: TableEmptyMatcher,
    pub table_stash_empty_matcher: TableEmptyMatcher,
    pub consumable_items_table_matcher: ConsumableItemsTableMatcher,
    pub monster_matcher: Tree,
    pub profile: Profile,
    pub item_filter: ItemsFilter,
    pub bot_settings: BotSettings,
    pub palette_transformer: PaletteTransformer,
    pub buffs: Buffs,
    pub spell_caster: SpellCaster,
    pub pixel_palette: PixelPalette,
    pub potion_seller_matcher: Tree,
    pub deckard_cain_matcher: Tree,
    pub logger: Logger,
    pub weapon_swapper: WeaponSwapper,
    pub game_screenshotter: GameScreenshotter,
    pub output_controller: OutputController,
    pub belt: Belt,
    pub game_interface_element_controller: GameInterfaceElementController,
    pub inventory: Table,
    pub inventory_table_reserved_cells: Table,
    pub map_sprite_matcher: MapMatcher,
    pub zone_name_converter: ZoneNameConverter,
}

impl Game {
    pub fn new(
        mut archives: Archives,
        file_io: FileIo,
        profile: Profile,
        bot_settings: BotSettings,
        game_screenshotter: GameScreenshotter,
        output_controller: OutputController,
    ) -> io::Result<Self> {
        let item_filter = file_io.load_items_filter(&profile.item_filter).unwrap();

        let act = profile.zone_to_farm.to_act();

        let font_dc6_bytes = archives.extract_font_16_bytes().unwrap();
        let font_dc6_file = font_dc6_bytes.parse();
        let font_char_map = get_non_control_ascii_char_font_map(&font_dc6_file);

        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);
        let quality_palettes = pal_pl2_bytes
            .extract_font_quality_palette_bytes()
            .get_palettes();

        let font_symbol_matcher = FontMatcher::new(&quality_palettes, &font_char_map);
        let table_inventory_empty_matcher =
            TableEmptyMatcher::new(&mut archives, TableType::Inventory);
        let table_stash_empty_matcher = TableEmptyMatcher::new(&mut archives, TableType::Stash);
        let consumable_items_table_matcher =
            ConsumableItemsTableMatcher::new(&mut archives, &font_char_map);

        let skill_names = &Self::get_skill_names(&profile);
        let skill_icon_getter = SkillIconGetter::new(&mut archives, skill_names);
        let buffs = Buffs::new(&profile.buffs, bot_settings.clone());

        let faster_cast_rate = WeaponSetData::new(
            profile.faster_cast_rate_weaponset_primary,
            profile.faster_cast_rate_weaponset_secondary,
        );

        let spell_caster = SpellCaster::new(
            &profile.keybindings.skills,
            profile.character_class,
            &faster_cast_rate,
        );

        let level_name = &profile.zone_to_farm.to_string();

        let string_tables = StringTables::new(&mut archives);
        let zone_name_converter = ZoneNameConverter::new(&string_tables);

        let npc_matcher_config =
            MonsterMatcherConfig::new_npc_matcher_config(act, profile.game_difficulty);

        let level_monsters_matcher_config = MonsterMatcherConfig::new_levels_monster_matcher_config(
            act,
            profile.game_difficulty,
            &bot_settings,
        );

        let potion_seller_matcher = get_monster_tree(
            &mut archives,
            &file_io,
            &act.get_potion_seller_name(),
            &npc_matcher_config,
            &zone_name_converter,
        )?;

        let deckard_cain_matcher = get_monster_tree(
            &mut archives,
            &file_io,
            &act.get_deckard_cain_monster_id(),
            &npc_matcher_config,
            &zone_name_converter,
        )?;

        let monster_matcher = get_monster_tree(
            &mut archives,
            &file_io,
            level_name,
            &level_monsters_matcher_config,
            &zone_name_converter,
        )?;

        if bot_settings.pre_cache_connected_areas {
            pre_cache_connected_areas(
                level_name,
                act,
                profile.game_difficulty,
                &bot_settings,
                &file_io,
                &archives,
                &zone_name_converter,
            );
        }

        let logger = Logger::new(&file_io, &bot_settings);

        let weapon_swapper = WeaponSwapper::new(
            &profile.keybindings.miscellaneous,
            &skill_icon_getter,
            &profile.left_skill_weaponset_primary,
            &profile.left_skill_weaponset_secondary,
        );

        let belt = Belt::new(
            profile.num_belt_columns_reserved_for_healing_potions,
            profile.num_belt_columns_reserved_for_mana_potions,
        );

        let game_interface_element_controller =
            GameInterfaceElementController::new(&profile.keybindings.game_interface_actions);

        let empty_inventory = Table::new(INVENTORY_TABLE_META_DATA);

        let inventory_table_reserved_cells = empty_inventory.clone();

        let area = zone_to_area(&profile.zone_to_farm.to_string());

        let map_sprite_matcher = MapMatcher::new(&area, &mut archives);

        Ok(Self {
            archives,
            file_io,
            font_symbol_matcher,
            table_inventory_empty_matcher,
            table_stash_empty_matcher,
            consumable_items_table_matcher,
            monster_matcher,
            profile,
            item_filter,
            palette_transformer,
            buffs,
            spell_caster,
            pixel_palette,
            potion_seller_matcher,
            deckard_cain_matcher,
            logger,
            weapon_swapper,
            bot_settings,
            game_screenshotter,
            output_controller,
            belt,
            game_interface_element_controller,
            inventory: empty_inventory,
            inventory_table_reserved_cells,
            map_sprite_matcher,
            zone_name_converter,
        })
    }

    fn get_skill_names(profile: &Profile) -> Vec<&str> {
        let mut skill_names = vec![
            profile.primary_attack_skill.as_str(),
            profile.left_skill_weaponset_primary.as_str(),
            profile.left_skill_weaponset_secondary.as_str(),
        ];

        for buff in &profile.buffs {
            skill_names.push(buff.skill.as_str());
        }

        skill_names
    }
}
