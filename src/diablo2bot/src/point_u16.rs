use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

use crate::{box_u16::BoxU16, point_u8::PointU8};

#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, Deserialize, Hash)]
pub struct PointU16 {
    pub row: u16,
    pub col: u16,
}

impl PointU16 {
    pub fn new(row: u16, col: u16) -> Self {
        Self { row, col }
    }

    pub fn len(self) -> u32 {
        u32::from(self.row) * u32::from(self.col)
    }

    pub fn get_points(self) -> Vec<Self> {
        let mut points = Vec::new();

        for row in 0..self.row {
            for col in 0..self.col {
                points.push(Self { row, col })
            }
        }

        points
    }

    pub fn is_in_area(self, area: BoxU16) -> bool {
        let top = area.offset.row;
        let bottom = top + area.dimensions.row;

        let left = area.offset.col;
        let right = left + area.dimensions.col;

        self.row >= top && self.row <= bottom && self.col >= left && self.col <= right
    }

    pub fn direction_to(&self, other: &PointU16) -> DirectionEnum {
        let row_diff = other.row as i16 - self.row as i16;
        let col_diff = other.col as i16 - self.col as i16;

        match (row_diff, col_diff) {
            (0, 0) => DirectionEnum::Same,
            (r, c) if r > 0 && c == 0 => DirectionEnum::South,
            (r, c) if r < 0 && c == 0 => DirectionEnum::North,
            (r, c) if r == 0 && c > 0 => DirectionEnum::East,
            (r, c) if r == 0 && c < 0 => DirectionEnum::West,
            (r, c) if r > 0 && c > 0 => DirectionEnum::SouthEast,
            (r, c) if r > 0 && c < 0 => DirectionEnum::SouthWest,
            (r, c) if r < 0 && c > 0 => DirectionEnum::NorthEast,
            (r, c) if r < 0 && c < 0 => DirectionEnum::NorthWest,
            _ => DirectionEnum::Same,
        }
    }

    pub fn move_in_direction(&self, direction: DirectionEnum, steps: u16) -> Vec<PointU16> {
        let (row_delta, col_delta) = match direction {
            DirectionEnum::North => (-1, 0),
            DirectionEnum::NorthEast => (-1, 1),
            DirectionEnum::East => (0, 1),
            DirectionEnum::SouthEast => (1, 1),
            DirectionEnum::South => (1, 0),
            DirectionEnum::SouthWest => (1, -1),
            DirectionEnum::West => (0, -1),
            DirectionEnum::NorthWest => (-1, -1),
            DirectionEnum::Same => (0, 0),
        };

        let mut points = Vec::new();
        let mut current_point = PointU16 {
            row: self.row,
            col: self.col,
        };

        for _ in 0..steps {
            let new_row = current_point.row as i16 + row_delta;
            let new_col = current_point.col as i16 + col_delta;

            if new_row < 0 || new_col < 0 {
                return points;
            }

            current_point = PointU16 {
                row: new_row as u16,
                col: new_col as u16,
            };

            points.push(current_point);
        }

        points
    }
}

impl Add for PointU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for PointU16 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Mul for PointU16 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            row: self.row * other.row,
            col: self.col * other.col,
        }
    }
}

impl From<PointU8> for PointU16 {
    fn from(p: PointU8) -> Self {
        PointU16 {
            row: u16::from(p.row),
            col: u16::from(p.col),
        }
    }
}

impl PartialOrd for PointU16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.row < other.row {
            Some(Ordering::Less)
        } else if self.row > other.row {
            Some(Ordering::Greater)
        } else if self.col < other.col {
            Some(Ordering::Less)
        } else if self.col > other.col {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for PointU16 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::ops::Add<PointU8> for PointU16 {
    type Output = Self;

    fn add(self, other: PointU8) -> Self {
        self + PointU16::from(other)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DirectionEnum {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Same,
}

impl DirectionEnum {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
            Self::Same => Self::Same,
        }
    }

    pub fn get_random_diagonal_direction() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=7) {
            0 => Self::North,
            1 => Self::NorthEast,
            2 => Self::East,
            3 => Self::SouthEast,
            4 => Self::South,
            5 => Self::SouthWest,
            6 => Self::West,
            _ => Self::NorthWest,
        }
    }
}
