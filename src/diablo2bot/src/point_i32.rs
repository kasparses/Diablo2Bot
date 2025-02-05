#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PointI32 {
    pub row: i32,
    pub col: i32,
}

impl PointI32 {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}
