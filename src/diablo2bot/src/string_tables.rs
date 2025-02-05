use std::collections::{HashMap, HashSet};

use crate::{
    constants::zone_names::ZONE_NAMES,
    mpq_archives::archives::{Archives, StringTableType},
};

pub struct StringTables {
    pub table: HashMap<String, String>,
}

const STRING_TABLE_TYPE: [StringTableType; 3] = [
    StringTableType::Data,
    StringTableType::Expansion,
    StringTableType::Patch,
];

impl StringTables {
    pub fn new(archives: &mut Archives) -> Self {
        let string_tables: Vec<HashMap<String, String>> = STRING_TABLE_TYPE
            .into_iter()
            .map(|s| archives.extract_string_table(s).unwrap().parse().dictionary)
            .collect();

        let combined_table = combine_string_tables(&string_tables);

        Self {
            table: combined_table,
        }
    }
}

fn combine_string_tables(string_tables: &[HashMap<String, String>]) -> HashMap<String, String> {
    let mut dict = HashMap::new();

    for string_table in string_tables {
        for (k, v) in string_table.iter() {
            dict.insert(k.to_string(), v.to_string());
        }
    }

    dict
}

#[derive(Clone)]
pub struct ZoneNameConverter {
    pub dictionary: HashMap<String, String>,
}

impl ZoneNameConverter {
    pub fn new(string_tables: &StringTables) -> Self {
        let inverted_hash_map = invert_hash_map(&string_tables.table);

        let mut hash_map = HashMap::new();

        let zone_names: HashSet<&str> = HashSet::from_iter(ZONE_NAMES.into_iter());

        for zone_name in zone_names {
            if let Some(value) = inverted_hash_map.get(zone_name) {
                hash_map.insert(zone_name.to_string(), value.clone());
            }
        }

        Self {
            dictionary: hash_map,
        }
    }

    pub fn get_default_level_name_from_english_level_name(&self, name: &str) -> String {
        self.dictionary[name].to_string()
    }
}

fn invert_hash_map(hash_map: &HashMap<String, String>) -> HashMap<String, String> {
    let mut inverted_map = HashMap::new();

    for (key, value) in hash_map {
        inverted_map.insert(value.clone(), key.clone());
    }

    inverted_map
}
