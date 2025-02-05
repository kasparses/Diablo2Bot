use crate::{
    belt::BeltItem,
    enums::{
        belt_item_type::BeltItemType, healing_potion_type::HealingPotionType,
        mana_potion_type::ManaPotionType, rejuvenation_potion_type::RejuvenationPotionType,
    },
};

use super::belt_item_names::{
    ANTIDOTE_POTION, FULL_REJUVENATION_POTION, GREATER_HEALING_POTION, GREATER_MANA_POTION,
    HEALING_POTION, LIGHT_HEALING_POTION, LIGHT_MANA_POTION, MANA_POTION, MINOR_HEALING_POTION,
    MINOR_MANA_POTION, REJUVENATION_POTION, SCROLL_OF_IDENTIFY, SCROLL_OF_TOWN_PORTAL,
    STAMINA_POTION, SUPER_HEALING_POTION, SUPER_MANA_POTION, THAWING_POTION,
};

pub const BELT_ITEMS: [BeltItem; 17] = [
    BeltItem {
        name: MINOR_HEALING_POTION,
        inventory_sprite_file_name: "invhp1",
        belt_item_type: BeltItemType::HealingPotion(HealingPotionType::Minor),
    },
    BeltItem {
        name: LIGHT_HEALING_POTION,
        inventory_sprite_file_name: "invhp2",
        belt_item_type: BeltItemType::HealingPotion(HealingPotionType::Light),
    },
    BeltItem {
        name: HEALING_POTION,
        inventory_sprite_file_name: "invhp3",
        belt_item_type: BeltItemType::HealingPotion(HealingPotionType::Standard),
    },
    BeltItem {
        name: GREATER_HEALING_POTION,
        inventory_sprite_file_name: "invhp4",
        belt_item_type: BeltItemType::HealingPotion(HealingPotionType::Greater),
    },
    BeltItem {
        name: SUPER_HEALING_POTION,
        inventory_sprite_file_name: "invhp5",
        belt_item_type: BeltItemType::HealingPotion(HealingPotionType::Super),
    },
    BeltItem {
        name: MINOR_MANA_POTION,
        inventory_sprite_file_name: "invmp1",
        belt_item_type: BeltItemType::ManaPotion(ManaPotionType::Minor),
    },
    BeltItem {
        name: LIGHT_MANA_POTION,
        inventory_sprite_file_name: "invmp2",
        belt_item_type: BeltItemType::ManaPotion(ManaPotionType::Light),
    },
    BeltItem {
        name: MANA_POTION,
        inventory_sprite_file_name: "invmp3",
        belt_item_type: BeltItemType::ManaPotion(ManaPotionType::Standard),
    },
    BeltItem {
        name: GREATER_MANA_POTION,
        inventory_sprite_file_name: "invmp4",
        belt_item_type: BeltItemType::ManaPotion(ManaPotionType::Greater),
    },
    BeltItem {
        name: SUPER_MANA_POTION,
        inventory_sprite_file_name: "invmp5",
        belt_item_type: BeltItemType::ManaPotion(ManaPotionType::Super),
    },
    BeltItem {
        name: REJUVENATION_POTION,
        inventory_sprite_file_name: "invvps",
        belt_item_type: BeltItemType::RejuvenationPotion(RejuvenationPotionType::Rejuvenation),
    },
    BeltItem {
        name: FULL_REJUVENATION_POTION,
        inventory_sprite_file_name: "invvpl",
        belt_item_type: BeltItemType::RejuvenationPotion(RejuvenationPotionType::FullRejuvenation),
    },
    BeltItem {
        name: STAMINA_POTION,
        inventory_sprite_file_name: "invwps",
        belt_item_type: BeltItemType::StaminaPotion,
    },
    BeltItem {
        name: ANTIDOTE_POTION,
        inventory_sprite_file_name: "invnps",
        belt_item_type: BeltItemType::AntidotePotion,
    },
    BeltItem {
        name: THAWING_POTION,
        inventory_sprite_file_name: "invyps",
        belt_item_type: BeltItemType::ThawingPotion,
    },
    BeltItem {
        name: SCROLL_OF_TOWN_PORTAL,
        inventory_sprite_file_name: "invbsc",
        belt_item_type: BeltItemType::ScrollOfTownPortal,
    },
    BeltItem {
        name: SCROLL_OF_IDENTIFY,
        inventory_sprite_file_name: "invrsc",
        belt_item_type: BeltItemType::ScrollOfIdentify,
    },
];
