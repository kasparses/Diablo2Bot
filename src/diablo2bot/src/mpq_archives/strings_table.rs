// https://sfsrealm.hopto.org/tbld2specs.html

use std::collections::HashMap;

pub struct StringsTableRaw {
    bytes: Vec<u8>,
}

struct StringsTableMetadata {
    num_strings: u16,
    first_string_offset: u32,
}

struct NullTerminatedSlice<'a> {
    data: &'a [u8],
}

pub struct StringsTable {
    pub dictionary: HashMap<String, String>,
}

impl StringsTableRaw {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn parse(&self) -> StringsTable {
        StringsTable::new(&self.bytes)
    }
}

impl StringsTableMetadata {
    fn from_bytes(bytes: &[u8]) -> Self {
        let num_strings_bytes: [u8; 2] =
            bytes[2..4].try_into().expect("slice with incorrect length");

        let num_strings = u16::from_le_bytes(num_strings_bytes);

        let first_string_offset_bytes: [u8; 4] = bytes[9..13]
            .try_into()
            .expect("slice with incorrect length");

        let first_string_offset = u32::from_le_bytes(first_string_offset_bytes);

        Self {
            num_strings,
            first_string_offset,
        }
    }
}

impl<'a> NullTerminatedSlice<'a> {
    fn get_next(bytes: &'a [u8], mut offset: usize) -> Self {
        let start = offset;

        loop {
            if bytes[offset] == 0 {
                break;
            }

            offset += 1;
        }

        let end = offset;

        Self {
            data: &bytes[start..end],
        }
    }
}

impl StringsTable {
    pub fn new(bytes: &[u8]) -> Self {
        let metadata = StringsTableMetadata::from_bytes(bytes);

        let mut dictionary: HashMap<String, String> = HashMap::new();

        let mut offset = metadata.first_string_offset as usize;

        for _ in 0..metadata.num_strings {
            let key = NullTerminatedSlice::get_next(bytes, offset);
            offset += key.data.len() + 1;

            let value = NullTerminatedSlice::get_next(bytes, offset);
            offset += value.data.len() + 1;

            let value = String::from_utf8_lossy(value.data).to_string();

            let key = String::from_utf8_lossy(key.data);

            dictionary.insert(key.to_string(), value);
        }

        Self { dictionary }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{file_io::FileIo, mpq_archives::archives::Archives, string_tables::StringTables};

    #[test]
    fn test_string_tables() {
        let file_io = FileIo::new();
        let system_settings = file_io.load_system_settings().unwrap();
        let mut archives = Archives::new(&system_settings.diablo2_folder_path);

        let now = Instant::now();
        let string_tables = StringTables::new(&mut archives);
        println!("elapsed: {} micros", now.elapsed().as_micros());

        println!("{}", string_tables.table.len());
    }
}
