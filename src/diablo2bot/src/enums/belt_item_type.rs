use super::{
    healing_potion_type::HealingPotionType, mana_potion_type::ManaPotionType,
    rejuvenation_potion_type::RejuvenationPotionType,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeltItemType {
    HealingPotion(HealingPotionType),
    ManaPotion(ManaPotionType),
    RejuvenationPotion(RejuvenationPotionType),
    StaminaPotion,
    AntidotePotion,
    ThawingPotion,
    ScrollOfTownPortal,
    ScrollOfIdentify,
}

#[derive(Debug)]
pub enum HealthManaPotionType {
    HealingPotion,
    ManaPotion,
    RejuvenationPotion,
}

impl BeltItemType {
    pub fn is_auto_pickup(self) -> bool {
        match self {
            Self::HealingPotion(_) | Self::ManaPotion(_) | Self::RejuvenationPotion(_) => true,
            Self::StaminaPotion
            | Self::AntidotePotion
            | Self::ThawingPotion
            | Self::ScrollOfTownPortal
            | Self::ScrollOfIdentify => false,
        }
    }

    pub fn is_equal_type(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::HealingPotion(_), Self::HealingPotion(_))
                | (Self::ManaPotion(_), Self::ManaPotion(_))
                | (Self::RejuvenationPotion(_), Self::RejuvenationPotion(_))
                | (Self::StaminaPotion, Self::StaminaPotion)
                | (Self::AntidotePotion, Self::AntidotePotion)
                | (Self::ThawingPotion, Self::ThawingPotion)
                | (Self::ScrollOfTownPortal, Self::ScrollOfTownPortal)
                | (Self::ScrollOfIdentify, Self::ScrollOfIdentify)
        )
    }
}
