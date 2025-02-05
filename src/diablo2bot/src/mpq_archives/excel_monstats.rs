use std::collections::HashMap;

pub struct ExcelMonstatsRawText {
    text: String,
}

impl ExcelMonstatsRawText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ExcelMonstats {
        ExcelMonstats::new(&self.text)
    }
}

pub struct Row<'raw_text> {
    pub id: &'raw_text str,
    pub name: &'raw_text str,
    pub code: &'raw_text str,
    pub palshift_id: u32,
    pub monstats2_id: &'raw_text str,
    pub spawn_id: Option<&'raw_text str>,
    pub minion_ids: [Option<&'raw_text str>; 2],
}

pub struct ExcelMonstats<'raw_text> {
    rows: Vec<Row<'raw_text>>,
    id_to_row_id: HashMap<&'raw_text str, usize>,
}

impl<'raw_text> ExcelMonstats<'raw_text> {
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

        let id_col_id = column_headers["Id"];
        let name_col_id = column_headers["NameStr"];
        let code_col_id = column_headers["Code"];
        let palshift_id_col_id = column_headers["TransLvl"];
        let monstats2_id_col_id = column_headers["MonStatsEx"];
        let spawn_id_col_id = column_headers["spawn"];
        let minion1_col_id = column_headers["minion1"];

        let mut row: Vec<&str> = vec![""; num_columns];

        let mut parsed_rows = Vec::new();

        for line in line_iter {
            for (i, column) in line.split(separator).enumerate() {
                row[i] = column;
            }

            let id = row[id_col_id];

            if id == "Expansion" {
                continue;
            }

            let name = row[name_col_id];
            let code = row[code_col_id];
            let palshift_id = row[palshift_id_col_id].parse().unwrap();
            let monstats2_id = row[monstats2_id_col_id];

            let spawn_id = row[spawn_id_col_id];

            let spawn_id = if spawn_id.is_empty() {
                None
            } else {
                Some(spawn_id)
            };

            let mut minion_ids = [None; 2];

            for i in 0..2 {
                let minion_id = row[minion1_col_id + i];

                if !minion_id.is_empty() {
                    minion_ids[i] = Some(minion_id);
                }
            }

            parsed_rows.push(Row {
                id,
                name,
                code,
                palshift_id,
                monstats2_id,
                spawn_id,
                minion_ids,
            });
        }

        let id_to_row_id = Self::get_id_to_row_id_map(&parsed_rows);

        Self {
            rows: parsed_rows,
            id_to_row_id,
        }
    }

    pub fn get_row(&self, id: &str) -> &'raw_text Row {
        let id = id.to_lowercase(); // All ids are lowercase
        let row_id = self.id_to_row_id[id.as_str()];

        &self.rows[row_id]
    }

    fn get_id_to_row_id_map(rows: &[Row<'raw_text>]) -> HashMap<&'raw_text str, usize> {
        let mut id_to_row_id = HashMap::new();

        for (i, row) in rows.iter().enumerate() {
            id_to_row_id.insert(row.id, i);
        }

        id_to_row_id
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{file_io::FileIo, mpq_archives::archives::Archives};

    #[test]
    fn test_excel_monstats() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let excel_monstats_raw_text = archives.extract_excel_monstats_raw_text().unwrap();
        let now = Instant::now();
        let excel_monstats = excel_monstats_raw_text.parse();
        println!("elapsed: {:?} micros", now.elapsed().as_micros());
        println!("excel_monstats.len(): {}", excel_monstats.rows.len());
    }
}
