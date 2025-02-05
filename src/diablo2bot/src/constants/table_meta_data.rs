use crate::{point_u16::PointU16, table::TableMetaData};

pub const MERCHANT_TABLE_META_DATA: TableMetaData = TableMetaData {
    cell_size: PointU16 { row: 29, col: 29 },
    table_size: PointU16 { row: 10, col: 10 },
    top_left_point: PointU16 { row: 124, col: 96 },
};

pub const BELT_TABLE_META_DATA: TableMetaData = TableMetaData {
    cell_size: PointU16 { row: 32, col: 31 },
    table_size: PointU16 { row: 4, col: 4 },
    top_left_point: PointU16 { row: 467, col: 423 },
};

pub const INVENTORY_TABLE_META_DATA: TableMetaData = TableMetaData {
    cell_size: PointU16 { row: 29, col: 29 },
    table_size: PointU16 { row: 4, col: 10 },
    top_left_point: PointU16 { row: 315, col: 417 },
};

pub const STASH_TABLE_META_DATA: TableMetaData = TableMetaData {
    cell_size: PointU16 { row: 29, col: 29 },
    table_size: PointU16 { row: 8, col: 6 },
    top_left_point: PointU16 { row: 143, col: 153 },
};
