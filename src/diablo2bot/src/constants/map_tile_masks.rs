pub const SINGLE: [[MapSpriteCell; 2]; 8] = [
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Wall, MapSpriteCell::Empty],
];

pub const SINGLE_HIGH: [[MapSpriteCell; 2]; 8] = [
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Wall, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
];

pub const EMPTY: [[MapSpriteCell; 2]; 8] = [
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
];

pub const LARGE: [[MapSpriteCell; 2]; 8] = [
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Wall, MapSpriteCell::Wall],
    [MapSpriteCell::Wall, MapSpriteCell::Wall],
    [MapSpriteCell::Wall, MapSpriteCell::Wall],
];

pub const OPENING: [[MapSpriteCell; 2]; 8] = [
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Empty, MapSpriteCell::Empty],
    [MapSpriteCell::Opening, MapSpriteCell::Opening],
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MapSpriteCell {
    Empty,
    Wall,
    Opening,
}

impl MapSpriteCell {
    pub fn is_walkable(self) -> bool {
        match self {
            MapSpriteCell::Empty | MapSpriteCell::Opening => true,
            MapSpriteCell::Wall => false,
        }
    }
}
