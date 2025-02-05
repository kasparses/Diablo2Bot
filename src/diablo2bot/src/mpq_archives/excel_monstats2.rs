use std::collections::HashMap;

use crate::{
    enums::{composit::Composit, mode::Mode},
    zone_monsters::CompositEquipments,
};

pub struct ExcelMonstats2RawText {
    text: String,
}

impl ExcelMonstats2RawText {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(&self) -> ExcelMonstats2 {
        ExcelMonstats2::new(&self.text)
    }
}

#[derive(Clone, Copy)]
pub struct CompositData<'raw_text> {
    composit: Composit,
    equipments: &'raw_text str,
}

pub struct Row<'raw_text> {
    pub id: &'raw_text str,
    pub base_weapon: &'raw_text str,
    pub modes: [Option<Mode>; 16],
    pub composits: [Option<CompositData<'raw_text>>; 16],
}

impl Row<'_> {
    pub fn get_modes(&self) -> Vec<Mode> {
        self.modes.into_iter().flatten().collect()
    }

    pub fn get_composit_equipments(&self) -> Vec<CompositEquipments> {
        let mut composits = Vec::new();

        for composit in self.composits.into_iter().flatten() {
            composits.push(
                self.get_composit_equipment(composit.composit, composit.equipments.to_string()),
            );
        }

        composits
    }

    pub fn get_composit_equipment(
        &self,
        composit: Composit,
        mut equipments: String,
    ) -> CompositEquipments {
        if equipments.starts_with('"') && equipments.ends_with('"') && equipments.len() >= 2 {
            equipments.pop();
            equipments.remove(0);
        }

        let mut equipments: Vec<String> = equipments
            .split(',')
            .filter(|s| *s != "nil")
            .map(|s| s.to_string())
            .collect();

        equipments.sort();
        equipments.dedup();

        CompositEquipments {
            composit,
            equipments,
        }
    }
}

pub struct ExcelMonstats2<'raw_text> {
    rows: Vec<Row<'raw_text>>,
    id_to_row_id: HashMap<&'raw_text str, usize>,
}

impl<'raw_text> ExcelMonstats2<'raw_text> {
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
        let base_weapon_col_id = column_headers["BaseW"];

        let mode_col_ids = [
            (Mode::DT, column_headers["mDT"]),
            (Mode::NU, column_headers["mNU"]),
            (Mode::WL, column_headers["mWL"]),
            (Mode::GH, column_headers["mGH"]),
            (Mode::A1, column_headers["mA1"]),
            (Mode::A2, column_headers["mA2"]),
            (Mode::BL, column_headers["mBL"]),
            (Mode::SC, column_headers["mSC"]),
            (Mode::S1, column_headers["mS1"]),
            (Mode::S2, column_headers["mS2"]),
            (Mode::S3, column_headers["mS3"]),
            (Mode::S4, column_headers["mS4"]),
            (Mode::DD, column_headers["mDD"]),
            (Mode::KB, column_headers["mKB"]),
            (Mode::SQ, column_headers["mSQ"]),
            (Mode::RN, column_headers["mRN"]),
        ];

        let composit_col_ids = [
            (Composit::HD, column_headers["HD"], column_headers["HDv"]),
            (Composit::TR, column_headers["TR"], column_headers["TRv"]),
            (Composit::LG, column_headers["LG"], column_headers["LGv"]),
            (Composit::RA, column_headers["RA"], column_headers["Rav"]), // For some reason the 'a' is lowercase in the csv file
            (Composit::LA, column_headers["LA"], column_headers["Lav"]), // For some reason the 'a' is lowercase in the csv file
            (Composit::RH, column_headers["RH"], column_headers["RHv"]),
            (Composit::LH, column_headers["LH"], column_headers["LHv"]),
            (Composit::SH, column_headers["SH"], column_headers["SHv"]),
            (Composit::S1, column_headers["S1"], column_headers["S1v"]),
            (Composit::S2, column_headers["S2"], column_headers["S2v"]),
            (Composit::S3, column_headers["S3"], column_headers["S3v"]),
            (Composit::S4, column_headers["S4"], column_headers["S4v"]),
            (Composit::S5, column_headers["S5"], column_headers["S5v"]),
            (Composit::S6, column_headers["S6"], column_headers["S6v"]),
            (Composit::S7, column_headers["S7"], column_headers["S7v"]),
            (Composit::S8, column_headers["S8"], column_headers["S8v"]),
        ];

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

            let base_weapon = row[base_weapon_col_id];

            let mut modes = [None; 16];

            let mut c = 0;
            for (mode, col_id) in mode_col_ids {
                let mode_str = row[col_id];
                if mode_str == "1" {
                    modes[c] = Some(mode);
                    c += 1;
                }
            }

            let mut composits = [None; 16];

            let mut c = 0;
            for (composit, has_composit_col_id, composit_equipment_col_id) in composit_col_ids {
                let has_composit = row[has_composit_col_id];
                if has_composit == "1" {
                    composits[c] = Some(CompositData {
                        composit,
                        equipments: row[composit_equipment_col_id],
                    });
                    c += 1;
                }
            }

            parsed_rows.push(Row {
                id,
                base_weapon,
                modes,
                composits,
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
    fn test_excel_monstats2() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let excel_monstats2_raw_text = archives.extract_excel_monstats2_raw_text().unwrap();
        let now = Instant::now();
        let excel_monstats2 = excel_monstats2_raw_text.parse();
        println!("elapsed: {:?} micros", now.elapsed().as_micros());
        println!("excel_monstats2.len(): {}", excel_monstats2.rows.len());
    }
}
