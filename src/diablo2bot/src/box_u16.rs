use crate::point_u16::PointU16;

#[derive(Clone, Copy)]
pub struct BoxU16 {
    pub offset: PointU16,
    pub dimensions: PointU16,
}

impl BoxU16 {
    pub fn get_middle_point(self) -> PointU16 {
        let row = (self.dimensions.row / 2) + self.offset.row;
        let col = (self.dimensions.col / 2) + self.offset.col;

        PointU16 { row, col }
    }

    pub fn offset(self, offset: PointU16) -> Self {
        Self {
            offset: self.offset + offset,
            dimensions: self.dimensions,
        }
    }

    pub fn get_window_box(self, window: PointU16) -> Self {
        assert!(window.col <= self.dimensions.col);
        assert!(window.row <= self.dimensions.row);

        Self {
            offset: PointU16::new(0, 0),
            dimensions: self.dimensions - window,
        }
    }

    pub fn iter_box_points(self) -> impl Iterator<Item = PointU16> {
        let row_range = self.offset.row..(self.offset.row + self.dimensions.row);
        let col_range = self.offset.col..(self.offset.col + self.dimensions.col);

        row_range.flat_map(move |r| col_range.clone().map(move |c| PointU16 { row: r, col: c }))
    }

    pub fn iter_box_points_with_step(self, step: PointU16) -> impl Iterator<Item = PointU16> {
        let row_range =
            (self.offset.row..(self.offset.row + self.dimensions.row)).step_by(step.row as usize);
        let col_range =
            (self.offset.col..(self.offset.col + self.dimensions.col)).step_by(step.col as usize);

        row_range.flat_map(move |r| col_range.clone().map(move |c| PointU16 { row: r, col: c }))
    }

    pub fn get_dimensions(self) -> PointU16 {
        self.dimensions
    }
}

impl From<PointU16> for BoxU16 {
    fn from(dimensions: PointU16) -> Self {
        Self {
            offset: PointU16 { row: 0, col: 0 },
            dimensions,
        }
    }
}
