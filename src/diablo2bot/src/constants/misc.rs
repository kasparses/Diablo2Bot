use crate::{
    enums::{composit::Composit, mode::Mode},
    point_u16::PointU16,
};

pub const MILLISECONDS_PER_FRAME: u64 = 40;
pub const MAX_ITEM_SIZE: PointU16 = PointU16 { row: 4, col: 2 };

pub const GAME_WINDOW_SIZE: PointU16 = PointU16 { row: 600, col: 800 };

pub const MAP_SPRITES_MATRIX_SIZE: PointU16 = PointU16 { row: 150, col: 100 };

pub const INVENTORY_WINDOW_OFFSET: PointU16 = PointU16 { row: 61, col: 400 };
pub const STASH_WINDOW_OFFSET: PointU16 = PointU16 { row: 61, col: 80 };

pub const COMPOSITS_TO_SKIP: [Composit; 10] = [
    Composit::RH,
    Composit::LH,
    Composit::S1,
    Composit::S2,
    Composit::S3,
    Composit::S4,
    Composit::S5,
    Composit::S6,
    Composit::S7,
    Composit::S8,
];

pub const MODES_TO_SKIP: [Mode; 8] = [
    Mode::DD, // Corpse animation
    Mode::DT, // Death animation
    Mode::KB, // Can be knocked back
    Mode::SQ, // Has skill sequence
    Mode::S1,
    Mode::S2,
    Mode::S3,
    Mode::S4,
];
