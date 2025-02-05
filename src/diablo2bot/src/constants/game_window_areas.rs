use crate::{box_u16::BoxU16, point_u16::PointU16};

pub const SKILL_BAR_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 553, col: 0 },
    dimensions: PointU16 { row: 47, col: 800 },
};

pub const MINI_PANEL_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 547, col: 342 },
    dimensions: PointU16 { row: 6, col: 114 },
};

pub const HEALTH_GLOBE_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 508, col: 0 },
    dimensions: PointU16 { row: 92, col: 109 },
};

pub const MANA_GLOBE_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 508, col: 695 },
    dimensions: PointU16 { row: 92, col: 105 },
};

pub const LIFE_TEXT_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 490, col: 0 },
    dimensions: PointU16 { row: 17, col: 140 },
};

pub const MANA_TEXT_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 490, col: 635 },
    dimensions: PointU16 { row: 17, col: 165 },
};

pub const LEFT_SKILL_ICON_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 553, col: 117 },
    dimensions: PointU16 { row: 47, col: 48 },
};

pub const ZONE_NAME_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 9, col: 600 },
    dimensions: PointU16 { row: 17, col: 200 },
};

pub const GAME_INFO_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 9, col: 635 },
    dimensions: PointU16 { row: 64, col: 160 },
};

pub const WAYPOINT_TEXT_AREA: BoxU16 = BoxU16 {
    offset: PointU16 { row: 90, col: 80 },
    dimensions: PointU16 { row: 350, col: 320 },
};

pub const NOISY_AREAS: [BoxU16; 7] = [
    GAME_INFO_AREA,
    LIFE_TEXT_AREA,
    MANA_TEXT_AREA,
    HEALTH_GLOBE_AREA,
    MANA_GLOBE_AREA,
    MINI_PANEL_AREA,
    SKILL_BAR_AREA,
];

pub const NOISY_AREAS_KEEP_HEALTH_MANA_TEXT: [BoxU16; 5] = [
    GAME_INFO_AREA,
    HEALTH_GLOBE_AREA,
    MANA_GLOBE_AREA,
    MINI_PANEL_AREA,
    SKILL_BAR_AREA,
];

// pub const LIFE_NUMBERS_AREA: BoxU16 = BoxU16{
//     offset: PointU16 { row: 492, col: 20 },
//     dimensions: PointU16 { row: 14, col: 120 },
// };

// pub const MANA_NUMBERS_AREA: BoxU16 = BoxU16{
//     offset: PointU16 { row: 492, col: 700 },
//     dimensions: PointU16 { row: 14, col: 99 },
// };

// pub const WAYPOINT_TEXT_AREA: BoxU16 = BoxU16{
//     offset: PointU16 { row: 90, col: 80 },
//     dimensions: PointU16 { row: 350, col: 320 },
// };
