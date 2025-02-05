use crate::dcc_decoder::Cell;

#[derive(Clone, Debug)]
pub struct Frame {
    pub dims: PointU16,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointU16 {
    pub row: u16,
    pub col: u16,
}

impl Frame {
    pub fn new(dims: PointU16) -> Self {
        let data = vec![0; dims.row as usize * dims.col as usize];
        Self { dims, data }
    }

    pub fn clear(&mut self) {
        for x in &mut self.data {
            *x = 0;
        }
    }

    pub fn set_cell_to_color(&mut self, cell: Cell, color: u8) {
        for row in cell.offset.row..cell.offset.row + cell.size.row {
            for col in cell.offset.col..cell.offset.col + cell.size.col {
                let idx = (u32::from(row) * u32::from(self.dims.col) + u32::from(col)) as usize;

                self.data[idx] = color;
            }
        }
    }

    pub fn copy_cell_internal(
        &mut self,
        cell_offset_src: PointU16,
        cell_offset_dst: PointU16,
        cell_size: PointU16,
    ) {
        for row in 0..u32::from(cell_size.row) {
            for col in 0..u32::from(cell_size.col) {
                let idx_src = ((u32::from(cell_offset_src.row) + row) * u32::from(self.dims.col)
                    + u32::from(cell_offset_src.col)
                    + col) as usize;
                let idx_dst = ((u32::from(cell_offset_dst.row) + row) * u32::from(self.dims.col)
                    + u32::from(cell_offset_dst.col)
                    + col) as usize;

                self.data[idx_dst] = self.data[idx_src];
            }
        }
    }

    pub fn copy_cell_external(&mut self, dst: &mut Self, cell: Cell) {
        for row in cell.offset.row..cell.offset.row + cell.size.row {
            for col in cell.offset.col..cell.offset.col + cell.size.col {
                let idx = (u32::from(row) * u32::from(self.dims.col) + u32::from(col)) as usize;

                dst.data[idx] = self.data[idx];
            }
        }
    }

    pub fn set_cell<F>(&mut self, cell: Cell, mut get_next_byte: F)
    where
        F: FnMut() -> u8,
    {
        for row in cell.offset.row..cell.offset.row + cell.size.row {
            for col in cell.offset.col..cell.offset.col + cell.size.col {
                let idx = (u32::from(row) * u32::from(self.dims.col) + u32::from(col)) as usize;

                self.data[idx] = get_next_byte();
            }
        }
    }
}
