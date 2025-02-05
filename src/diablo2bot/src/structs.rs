use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enums::quality::Quality;
use crate::matrix::Matrix;
use crate::point_u16::PointU16;

#[derive(Clone)]
pub struct MatrixAndPoints {
    pub matrix: Matrix,
    pub point_values: Vec<PointValue>,
}

#[derive(Clone)]
pub struct MatrixAndPoints2 {
    pub sprite_id: u32,
    pub matrix: Matrix,
    pub point_values: Vec<PointValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Default)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Copy, Clone)]
pub struct PointValue {
    pub point: PointU16,
    pub value: u8,
}

pub struct PointIndex {
    pub sprite_id: u32,
    pub point: PointU16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemFilter {
    pub grey: bool,
    pub common: bool,
    pub magic: bool,
    pub rare: bool,
    pub set: bool,
    pub unique: bool,
    pub rune: bool,
    pub row_size: u8,
    pub col_size: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemsFilter {
    pub items: HashMap<String, ItemFilter>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    pub name: String,
    pub quality: Quality,
    pub point: PointU16,
}

#[derive(Clone, Copy)]
pub struct QualityCharacter {
    pub char: char,
    pub width: u8,
    pub quality: Quality,
}

#[derive(Clone)]
pub struct Character {
    pub char: QualityCharacter,
    pub matrix_and_points: MatrixAndPoints,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrieOutput<Output>
where
    Output: Clone,
{
    pub point: PointU16,
    pub output: Output,
}

pub trait TrieDataTrait {
    type Output: Clone;

    fn get_matrix(&self) -> &Matrix;
    fn get_point_values(&self) -> &Vec<PointValue>;
    fn get_output(&self, point: PointU16) -> TrieOutput<Self::Output>;
}

impl TrieDataTrait for Character {
    type Output = QualityCharacter;

    fn get_matrix(&self) -> &Matrix {
        &self.matrix_and_points.matrix
    }

    fn get_point_values(&self) -> &Vec<PointValue> {
        &self.matrix_and_points.point_values
    }

    fn get_output(&self, point: PointU16) -> TrieOutput<Self::Output> {
        TrieOutput {
            point,
            output: self.char,
        }
    }
}

#[derive(Clone)]
pub struct ConsumableItem {
    pub name: String,
    pub matrix_and_points: MatrixAndPoints,
}

impl TrieDataTrait for ConsumableItem {
    type Output = String;

    fn get_matrix(&self) -> &Matrix {
        &self.matrix_and_points.matrix
    }

    fn get_point_values(&self) -> &Vec<PointValue> {
        &self.matrix_and_points.point_values
    }

    fn get_output(&self, point: PointU16) -> TrieOutput<Self::Output> {
        TrieOutput {
            point,
            output: self.name.clone(),
        }
    }
}
