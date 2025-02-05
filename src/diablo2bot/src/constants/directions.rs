use crate::{point_i32::PointI32, zone_traveller::Direction};

pub const DIRECTIONS2: [Direction; 4] = [
    // Up
    Direction {
        point: PointI32 { row: -1, col: 0 },
        is_vertical: true,
    },
    // Right
    Direction {
        point: PointI32 { row: 0, col: 1 },
        is_vertical: false,
    },
    // Down
    Direction {
        point: PointI32 { row: 1, col: 0 },
        is_vertical: true,
    },
    // Left
    Direction {
        point: PointI32 { row: 0, col: -1 },
        is_vertical: false,
    },
];

pub const DIRECTIONS: [PointI32; 4] = [
    PointI32 { row: -1, col: 0 }, // Up
    PointI32 { row: 0, col: 1 },  // Right
    PointI32 { row: 1, col: 0 },  // Down
    PointI32 { row: 0, col: -1 }, // Left
];
