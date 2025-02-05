use std::collections::HashMap;

pub struct ExcelAutomapRawText {
    text: String,
}

impl ExcelAutomapRawText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ExcelAutomap {
        ExcelAutomap::new(&self.text)
    }
}

struct Row<'raw_text> {
    level_name: &'raw_text str,
    cells: [Option<u32>; 4],
}

pub struct ExcelAutomap<'raw_text> {
    rows: Vec<Row<'raw_text>>,
}

impl<'raw_text> ExcelAutomap<'raw_text> {
    pub fn new(text: &'raw_text str) -> Self {
        let separator = '\t';

        let mut line_iter = text.split("\r\n");

        let mut column_headers = HashMap::new();
        let mut num_columns = 0;

        if let Some(header_line) = line_iter.next() {
            for (i, header) in header_line.split(separator).enumerate() {
                num_columns += 1;
                column_headers.insert(header, i);
            }
        }

        let level_name_col_id = column_headers["LevelName"];

        let cell_col_ids = [
            column_headers["Cel1"],
            column_headers["Cel2"],
            column_headers["Cel3"],
            column_headers["Cel4"],
        ];

        let mut row: Vec<&str> = vec![""; num_columns];

        let mut parsed_rows = Vec::new();

        for line in line_iter {
            for (i, column) in line.split(separator).enumerate() {
                row[i] = column;
            }

            let level_name = row[level_name_col_id];

            if level_name == "Expansion" {
                continue;
            }

            let mut cells = [None; 4];

            for (i, cell_col_id) in cell_col_ids.iter().enumerate() {
                cells[i] = match row[*cell_col_id].parse::<i32>() {
                    Ok(number) => {
                        if number > 0 {
                            Some(number as u32)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                };
            }

            parsed_rows.push(Row { level_name, cells });
        }

        Self { rows: parsed_rows }
    }

    pub fn get_map_sprite_ids_for_area(&self, area_name: &str) -> Vec<u32> {
        let mut ids = Vec::new();

        for row in &self.rows {
            if row.level_name == area_name {
                for id in row.cells.into_iter().flatten() {
                    ids.push(id);
                }
            }
        }

        ids.sort();
        ids.dedup();

        ids
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{file_io::FileIo, mpq_archives::archives::Archives};

    #[test]
    fn test_excel_automap() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let excel_automap_raw_text = archives.extract_excel_automap_raw_text().unwrap();
        let now = Instant::now();
        let excel_automap = excel_automap_raw_text.parse();
        println!("elapsed: {:?} micros", now.elapsed().as_micros());
        println!("{}", excel_automap.rows.len());
    }
}
