use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum State {
    AutomapFadeNo,
    AutomapOptionsMenu,
    AutomapShowPartyNo,
    AutomapSizeFull,
    DifficultyMenu,
    MainMenu,
    HasDied,
    InGame,
    LightingQualityLow,
    Menu,
    OptionsMenu,
    SinglePlayerMenu,
    Stash,
    VideoOptionsMenu,
    WaypointMenu,
    InventoryOpen,
    BeltOpen,
    MerchantTradeWindowOpen,
}
