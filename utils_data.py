from enums_diablo import ACT, WaypointZone

acts_waypointzones = {
    ACT.ACT1: {WaypointZone.ROGUE_ENCAMPMENT,           WaypointZone.COLD_PLAINS,           WaypointZone.STONY_FIELD,       WaypointZone.DARK_WOOD,                 WaypointZone.BLACK_MARSH,   WaypointZone.OUTER_CLOISTER,    WaypointZone.JAIL_LEVEL_1,          WaypointZone.INNER_CLOISTER,    WaypointZone.CATACOMBS_LEVEL_2},
    ACT.ACT2: {WaypointZone.LUT_GHOLEIN,                WaypointZone.SEWERS_LEVEL_2,        WaypointZone.DRY_HILLS,         WaypointZone.HALLS_OF_THE_DEAD_LEVEL_2, WaypointZone.FAR_OASIS,     WaypointZone.LOST_CITY,         WaypointZone.PALACE_CELLAR_LEVEL_1, WaypointZone.ARCANE_SANCTUARY,  WaypointZone.CANYON_OF_THE_MAGI},
    ACT.ACT3: {WaypointZone.KURAST_DOCKS,               WaypointZone.SPIDER_FOREST,         WaypointZone.GREAT_MARSH,       WaypointZone.FLAYER_JUNGLE,             WaypointZone.LOWER_KURAST,  WaypointZone.KURAST_BAZAAR,     WaypointZone.UPPER_KURAST,          WaypointZone.TRAVINCAL,         WaypointZone.DURANCE_OF_HATE_LEVEL_2},
    ACT.ACT4: {WaypointZone.THE_PANDEMONIUM_FORTRESS,   WaypointZone.CITY_OF_THE_DAMNED,    WaypointZone.RIVER_OF_FLAME},
    ACT.ACT5: {WaypointZone.HARROGATH,                  WaypointZone.FRIGID_HIGHLANDS,      WaypointZone.ARREAT_PLATEAU,    WaypointZone.CRYSTALLINE_PASSAGE,       WaypointZone.GLACIAL_TRAIL, WaypointZone.HALLS_OF_PAIN,     WaypointZone.FROZEN_TUNDRA,         WaypointZone.THE_ANCIENTS_WAY,  WaypointZone.WORLDSTONE_KEEP_LEVEL_2}
    }

acts_zones = {
    ACT.ACT1: ("Rogue Encampment", "Blood Moor", "Cold Plains", "Stony Field", "Dark Wood", "Black Marsh", "Tamoe Highland", "Den of Evil", "Cave Level 1", "Underground Passage Level 1", "Hole Level 1", "Pit Level 1", "Cave Level 2", "Underground Passage Level 2", "Hole Level 2", "Pit Level 2", "Burial Grounds", "Crypt", "Mausoleum", "Tower Cellar Level 1", "Tower Cellar Level 2", "Tower Cellar Level 3", "Tower Cellar Level 4", "Tower Cellar Level 5", "Monastery Gate", "Outer Cloister", "Barracks", "Jail Level 1", "Jail Level 2", "Jail Level 3", "Inner Cloister", "Cathedral", "Catacombs Level 1", "Catacombs Level 2", "Catacombs Level 3", "Catacombs Level 4", "Tristram", "Secret Cow Level"),
    ACT.ACT2: ("Lut Gholein", "Rocky Waste", "Dry Hills", "Far Oasis", "Lost City", "Valley of Snakes", "Canyon of the Magi", "Sewers Level 1", "Sewers Level 2", "Sewers Level 3", "Harem Level 1", "Harem Level 2", "Palace Cellar Level 1", "Palace Cellar Level 2", "Palace Cellar Level 3", "Stony Tomb Level 1", "Halls of the Dead Level 1", "Halls of the Dead Level 2", "Claw Viper Temple Level 1", "Stony Tomb Level 2", "Halls of the Dead Level 3", "Claw Viper Temple Level 2", "Maggot Lair Level 1", "Maggot Lair Level 2", "Maggot Lair Level 3", "Ancient Tunnels", "Tal Rasha s Tomb", "Tal Rasha s Chamber", "Arcane Sanctuary"),
    ACT.ACT3: ("Kurast Docks", "Spider Forest", "Great Marsh", "Flayer Jungle", "Lower Kurast", "Kurast Bazaar", "Upper Kurast", "Kurast Causeway", "Travincal", "Spider Cave", "Arachnid Lair", "Spider Cavern", "Swampy Pit Level 1", "Swampy Pit Level 2", "Flayer Dungeon Level 1", "Flayer Dungeon Level 2", "Swampy Pit Level 3", "Flayer Dungeon Level 3", "Sewers Level 1", "Sewers Level 2", "Ruined Temple", "Disused Fane", "Forgotten Reliquary", "Forgotten Temple", "Ruined Fane", "Disused Reliquary", "Durance of Hate Level 1", "Durance of Hate Level 2", "Durance of Hate Level 3"),
    ACT.ACT4: ("The Pandemonium Fortress", "Outer Steppes", "Plains of Despair", "City of the Damned", "River of Flame", "Chaos Sanctuary"),
    ACT.ACT5: ("Harrogath", "Bloody Foothills", "Frigid Highlands", "Arreat Plateau", "Crystalline Passage", "Frozen River", "Glacial Trail", "Drifter Cavern", "Frozen Tundra", "The Ancients", "Icy Cellar", "Arreat Summit", "Nihlathaks Temple", "Halls of Anguish", "Halls of Pain", "Halls of Vaught", "Abaddon", "Pit of Acheron", "Infernal Pit", "Worldstone Keep Level 1", "Worldstone Keep Level 2", "Worldstone Keep Level 3", "Throne of Destruction", "Worldstone Chamber")
    }

waypoint_zones_monster_mapping = {
    WaypointZone.ROGUE_ENCAMPMENT: "",
    WaypointZone.COLD_PLAINS: "act_1_wilderness_1",
    WaypointZone.STONY_FIELD: "act_1_wilderness_1",
    WaypointZone.DARK_WOOD: "act_1_wilderness_2",
    WaypointZone.BLACK_MARSH: "act_1_wilderness_2",
    WaypointZone.OUTER_CLOISTER: "",
    WaypointZone.JAIL_LEVEL_1: "act_1_cellar",
    WaypointZone.INNER_CLOISTER: "",
    WaypointZone.CATACOMBS_LEVEL_2: "act_1_cellar",
    WaypointZone.LUT_GHOLEIN: "",
    WaypointZone.SEWERS_LEVEL_2: "",
    WaypointZone.DRY_HILLS: "act_2_desert",
    WaypointZone.HALLS_OF_THE_DEAD_LEVEL_2: "",
    WaypointZone.FAR_OASIS: "act_2_desert",
    WaypointZone.LOST_CITY: "act_2_desert",
    WaypointZone.PALACE_CELLAR_LEVEL_1: "",
    WaypointZone.ARCANE_SANCTUARY: "",
    WaypointZone.CANYON_OF_THE_MAGI: "",
    WaypointZone.KURAST_DOCKS: "",
    WaypointZone.SPIDER_FOREST: "act_3_jungle",
    WaypointZone.GREAT_MARSH: "act_3_jungle",
    WaypointZone.FLAYER_JUNGLE: "act_3_jungle",
    WaypointZone.LOWER_KURAST: "act_3_bazaar",
    WaypointZone.KURAST_BAZAAR: "act_3_bazaar",
    WaypointZone.UPPER_KURAST: "act_3_bazaar",
    WaypointZone.TRAVINCAL: "act_3_bazaar",
    WaypointZone.DURANCE_OF_HATE_LEVEL_2: "",
    WaypointZone.THE_PANDEMONIUM_FORTRESS: "",
    WaypointZone.CITY_OF_THE_DAMNED: "",
    WaypointZone.RIVER_OF_FLAME: "",
    WaypointZone.HARROGATH: "",
    WaypointZone.FRIGID_HIGHLANDS: "",
    WaypointZone.ARREAT_PLATEAU: "",
    WaypointZone.CRYSTALLINE_PASSAGE: "",
    WaypointZone.GLACIAL_TRAIL: "",
    WaypointZone.HALLS_OF_PAIN: "",
    WaypointZone.FROZEN_TUNDRA: "",
    WaypointZone.THE_ANCIENTS_WAY: "",
    WaypointZone.WORLDSTONE_KEEP_LEVEL_2: ""
}

items_that_can_be_picked_up_with_telekinesis = (
    "Rejuvenation Potion", "Full Rejuvenation Potion",
    "Minor Healing Potion", "Light Healing Potion", "Healing Potion", "Greater Healing Potion", "Super Healing Potion",
    "Minor Mana Potion", "Light Mana Potion", "Mana Potion", "Greater Mana Potion", "Super Mana Potion",
    "Stamina Potion", "Thawing Potion", "Antidote Potion",
    "Scroll of Town Portal", "Scroll of Identify",
    "Exploding Potion",
    "Bolts", "Arrows",
    "Key"
    )