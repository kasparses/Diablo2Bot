use crate::{
    constants::{
        misc::{GAME_WINDOW_SIZE, INVENTORY_WINDOW_OFFSET, STASH_WINDOW_OFFSET},
        table_meta_data::{INVENTORY_TABLE_META_DATA, STASH_TABLE_META_DATA},
    },
    enums::table_type::TableType,
    matrix::Matrix,
    mpq_archives::archives::Archives,
    table::{InventoryTable, Table, TableMetaData},
};

pub struct TableEmptyMatcher {
    empty_table: InventoryTable,
}

impl TableEmptyMatcher {
    pub fn new(archives: &mut Archives, table_type: TableType) -> Self {
        let empty_cells_matrix = Self::get_empty_cells(archives, table_type);

        let table_meta_data = Self::_get_table_meta_data(table_type);
        let empty_table = InventoryTable::new(table_meta_data, &empty_cells_matrix);

        Self { empty_table }
    }

    fn get_empty_cells(archives: &mut Archives, table_type: TableType) -> Matrix {
        match table_type {
            TableType::Inventory => {
                let inventory = archives
                    .extract_inventory_dc6_bytes()
                    .unwrap()
                    .parse()
                    .convert_to_matrix();

                let mut full_screen_inventory = Matrix::new_empty(GAME_WINDOW_SIZE);
                full_screen_inventory.insert_sub_matrix(INVENTORY_WINDOW_OFFSET, &inventory);

                full_screen_inventory
            }
            TableType::Stash => {
                let stash = archives
                    .extract_stash_dc6_bytes()
                    .unwrap()
                    .parse()
                    .convert_to_matrix();

                let mut full_screen_stash = Matrix::new_empty(GAME_WINDOW_SIZE);
                full_screen_stash.insert_sub_matrix(STASH_WINDOW_OFFSET, &stash);

                full_screen_stash
            }
        }
    }

    fn _get_table_meta_data(table_type: TableType) -> TableMetaData {
        match table_type {
            TableType::Inventory => INVENTORY_TABLE_META_DATA,
            TableType::Stash => STASH_TABLE_META_DATA,
        }
    }

    pub fn get_table_meta_data(&self) -> TableMetaData {
        self.empty_table.meta_data
    }

    pub fn match_from_matrix(&self, matrix: &Matrix) -> Table {
        let img_table = InventoryTable::new(self.empty_table.meta_data, matrix);
        let mut table = Table::new(self.empty_table.meta_data);

        for row in 0..self.empty_table.meta_data.table_size.row {
            for col in 0..self.empty_table.meta_data.table_size.col {
                let cell_matrix = &img_table.cells[row as usize][col as usize];
                let inventory_cell_matrix = &self.empty_table.cells[row as usize][col as usize];
                if cell_matrix.get_diff_percentage(inventory_cell_matrix) > 0.2 {
                    table.cells[row as usize][col as usize] = Some(String::from("Some"));
                }
            }
        }

        table
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    };

    use crate::{
        enums::act::Act,
        file_io::FileIo,
        image::Image,
        mpq_archives::archives::Archives,
        pal_pl2::PaletteTransformer,
        table::Table,
        test_utils::test_utils::{get_directory, read_json},
    };

    use super::TableEmptyMatcher;

    fn test_empty_matcher(
        archives: &mut Archives,
        table_type: super::TableType,
        palette_transformer: &PaletteTransformer,
        test_data_path: &PathBuf,
        test_data_folder_name: &str,
    ) {
        let dir = get_directory(
            test_data_path.as_os_str().to_str().unwrap(),
            test_data_folder_name,
        );

        let empty_matcher = TableEmptyMatcher::new(archives, table_type);

        for test_folder in dir.subdirectories.iter() {
            let img = Image::load_image(&Path::new(&format!("{}/inventory.png", test_folder.path)));
            let matrix = img.to_matrix(&palette_transformer);

            let table = empty_matcher.match_from_matrix(&matrix);
            let expected_table: Result<Table, _> = read_json(std::path::Path::new(&format!(
                "{}/inventory.json",
                test_folder.path
            )));

            match expected_table {
                Ok(expected_table) => {
                    assert_eq!(table, expected_table);
                }
                Err(err) => {
                    if let std::io::ErrorKind::NotFound = err.kind() {
                        eprintln!(
                            "File not found: {} - Saving results from current test as file",
                            err
                        );

                        table.print();

                        File::create(&format!("{}/inventory.json", test_folder.path))
                            .unwrap()
                            .write_all(serde_json::to_string_pretty(&table).unwrap().as_bytes())
                            .unwrap();
                    } else {
                        panic!("Error reading JSON file: {}", err);
                    }
                }
            }
        }
    }

    #[test]
    fn test_inventory_matcher() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let act = Act::Act1;
        let pal_pl2_bytes = archives.extract_pal_pl2_bytes(act.into()).unwrap();
        let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
        let pixel_palette = act_palette_bytes.get_pixel_palette();
        let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

        test_empty_matcher(
            &mut archives,
            super::TableType::Inventory,
            &palette_transformer,
            &file_io.root.join("test_data").join("table_cells"),
            "inventory",
        );

        test_empty_matcher(
            &mut archives,
            super::TableType::Stash,
            &palette_transformer,
            &file_io.root.join("test_data").join("table_cells"),
            "stash",
        );
    }
}
