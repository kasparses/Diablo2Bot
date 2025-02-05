use serde::{Deserialize, Serialize};

use crate::{box_u16::BoxU16, matrix::Matrix, point_u16::PointU16};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct TableMetaData {
    pub top_left_point: PointU16,
    pub table_size: PointU16,
    pub cell_size: PointU16,
}

impl TableMetaData {
    pub fn get_point(&self, cell: PointU16) -> PointU16 {
        self.top_left_point + cell * self.cell_size
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Table {
    pub cells: Vec<Vec<Option<String>>>,
}

pub struct InventoryTable {
    pub meta_data: TableMetaData,
    pub cells: Vec<Vec<Matrix>>,
}

impl InventoryTable {
    pub fn new(meta_data: TableMetaData, img: &Matrix) -> Self {
        let mut cells = Vec::new();
        for row in 0..meta_data.table_size.row {
            let mut row_cells = Vec::new();

            for col in 0..meta_data.table_size.col {
                let image_offset = PointU16 {
                    row: meta_data.top_left_point.row + (row * (meta_data.cell_size.row)),
                    col: meta_data.top_left_point.col + (col * (meta_data.cell_size.col)),
                };

                let matrix = img.get_sub_matrix(BoxU16 {
                    offset: image_offset,
                    dimensions: meta_data.cell_size,
                });

                row_cells.push(matrix);
            }
            cells.push(row_cells);
        }
        Self { meta_data, cells }
    }
}

impl Table {
    pub fn new(meta_data: TableMetaData) -> Self {
        Self {
            cells: Self::get_empty_cells(meta_data),
        }
    }

    fn get_empty_cells(meta_data: TableMetaData) -> Vec<Vec<Option<String>>> {
        vec![vec![None; meta_data.table_size.col as usize]; meta_data.table_size.row as usize]
    }

    fn _has_empty_cell_area(&self, offset: PointU16, dimensions: PointU16) -> bool {
        for row in offset.row..offset.row + dimensions.row {
            if usize::from(row) >= self.cells.len() {
                return false;
            }

            for col in offset.col..offset.col + dimensions.col {
                if usize::from(col) >= self.cells[0].len() {
                    return false;
                }

                if self.cells[row as usize][col as usize].is_some() {
                    return false;
                }
            }
        }

        true
    }

    pub fn has_empty_cell_area(&self, dimensions: PointU16) -> bool {
        for row in 0..self.cells.len() {
            for col in 0..self.cells[0].len() {
                if self._has_empty_cell_area(PointU16::new(row as u16, col as u16), dimensions) {
                    return true;
                }
            }
        }

        false
    }

    pub fn find_item_placement_in_cell_area(
        &self,
        num_rows: u32,
        num_cols: u32,
    ) -> Option<PointU16> {
        for row in 0..self.cells.len() - num_rows as usize + 1 {
            for col in 0..self.cells[0].len() - num_cols as usize + 1 {
                let mut has_space = true;

                for item_row in 0..num_rows as usize {
                    for item_col in 0..num_cols as usize {
                        if self.cells[row + item_row][col + item_col].is_some() {
                            has_space = false;
                            break;
                        }
                    }

                    if !has_space {
                        break;
                    }
                }

                if has_space {
                    return Some(PointU16::new(row as u16, col as u16));
                }
            }
        }

        None
    }

    pub fn has_space_for_item(&self, num_rows: u32, num_cols: u32) -> bool {
        self.find_item_placement_in_cell_area(num_rows, num_cols)
            .is_some()
    }
}
