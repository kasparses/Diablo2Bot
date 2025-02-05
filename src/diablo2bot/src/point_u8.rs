use crate::point_u16::PointU16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointU8 {
    pub row: u8,
    pub col: u8,
}

impl From<PointU16> for PointU8 {
    fn from(p: PointU16) -> Self {
        PointU8 {
            row: p.row as u8,
            col: p.col as u8,
        }
    }
}
