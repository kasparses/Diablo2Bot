use std::ops::{Add, Sub};

use serde::Deserialize;

use crate::constants::misc::MILLISECONDS_PER_FRAME;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Frames(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Milliseconds(pub u64);

impl From<Frames> for Milliseconds {
    fn from(value: Frames) -> Self {
        Milliseconds(value.0 * MILLISECONDS_PER_FRAME)
    }
}

impl Add for Milliseconds {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Milliseconds {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}
