use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::{
    bot_settings::BotSettings,
    pattern_matcher_monsters::{TreeCacheFiles, TreeCacheFilesBorrowed},
    profile::{Profile, SystemSettings},
    structs::ItemsFilter,
};

const SETTINGS: &str = "settings";
const CACHE: &str = "cache";
const TOML: &str = "toml";

impl TreeCacheFiles {
    const TREE_DATA_FILE_NAME: &'static str = "tree_data.bin";
    const MATRICES_DATA_FILE_NAME: &'static str = "matrices_data.bin";
    const MATRICES_PALETTES_FILE_NAME: &'static str = "matrices_palettes.bin";
}

#[derive(Clone)]
pub struct FileIo {
    pub root: PathBuf,
}

impl FileIo {
    pub fn new() -> Self {
        Self {
            root: get_project_root_directory_path(),
        }
    }

    pub fn load_system_settings(&self) -> io::Result<SystemSettings> {
        read_toml(&self.root.join(SETTINGS).join("system_settings"))
    }

    pub fn load_bot_settings(&self) -> io::Result<BotSettings> {
        read_toml(&self.root.join(SETTINGS).join("bot_settings"))
    }

    pub fn load_profile(&self, profile_name: &str) -> io::Result<Profile> {
        read_toml(&self.root.join(SETTINGS).join("profiles").join(profile_name))
    }

    pub fn load_items_filter(&self, item_filter_name: &str) -> io::Result<ItemsFilter> {
        read_toml(
            &self
                .root
                .join(SETTINGS)
                .join("item_filters")
                .join(item_filter_name),
        )
    }

    pub fn has_monster_matcher_cache_folder(&self, name: &str) -> bool {
        self.get_monster_matcher_cache_folder_path(name).exists()
    }

    pub fn load_monster_matcher_cache_files(&self, name: &str) -> io::Result<TreeCacheFiles> {
        let folder = self.get_monster_matcher_cache_folder_path(name);

        let tree_data = read_bytes(&folder.join(TreeCacheFiles::TREE_DATA_FILE_NAME))?;

        let matrices_data = read_bytes(&folder.join(TreeCacheFiles::MATRICES_DATA_FILE_NAME))?;

        let matrices_palettes =
            read_bytes(&folder.join(TreeCacheFiles::MATRICES_PALETTES_FILE_NAME))?;

        Ok(TreeCacheFiles {
            tree_data,
            matrices_data,
            matrices_palettes,
        })
    }

    pub fn save_monster_matcher_cache_files(
        &self,
        name: &str,
        cache_files: &TreeCacheFilesBorrowed,
    ) -> io::Result<()> {
        let folder = self.get_monster_matcher_cache_folder_path(name);

        fs::create_dir_all(&folder)?;

        fs::write(
            folder.join(TreeCacheFiles::TREE_DATA_FILE_NAME),
            cache_files.tree_data,
        )?;

        fs::write(
            folder.join(TreeCacheFiles::MATRICES_DATA_FILE_NAME),
            cache_files.matrices_data,
        )?;

        fs::write(
            folder.join(TreeCacheFiles::MATRICES_PALETTES_FILE_NAME),
            cache_files.matrices_palettes,
        )?;

        Ok(())
    }

    fn get_monster_matcher_cache_folder_path(&self, name: &str) -> PathBuf {
        self.root.join(CACHE).join("monster_matcher").join(name)
    }
}

fn get_project_root_directory_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();

    // Continue to loop up the directory tree until we find the project path
    loop {
        let mut path_ = path.clone();
        path_.push(SETTINGS);
        if path_.exists() {
            break;
        }

        let success = path.pop();
        if !success {
            panic!("Could not find project path");
        }
    }

    path
}

fn read_bytes(file_path: &Path) -> io::Result<Vec<u8>> {
    fs::read(file_path)
}

fn read_as_string(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn get_toml_file_path(file_path: &Path) -> PathBuf {
    let mut path_buf: PathBuf = file_path.into();

    if path_buf.extension() != Some(OsStr::new(TOML)) {
        path_buf.set_extension(TOML);
    }

    path_buf
}

fn read_toml<T: DeserializeOwned>(file_path: &Path) -> io::Result<T> {
    let file_path = get_toml_file_path(file_path);

    let text = read_as_string(&file_path)?;

    let data = toml::from_str(&text).unwrap();

    Ok(data)
}
