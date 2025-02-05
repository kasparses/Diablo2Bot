use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{bot_settings::BotSettings, file_io::FileIo, image::Image};

const LOG_FOLDER: &str = "logs";

pub struct Logger {
    log_folder_path: PathBuf,
    path_file_ids: HashMap<PathBuf, u32>,
    save_logs: bool,
}

impl Logger {
    pub fn new(file_io: &FileIo, bot_settings: &BotSettings) -> Self {
        let log_folder_path = file_io.root.join(LOG_FOLDER);

        fs::create_dir_all(&log_folder_path).unwrap();

        Self {
            log_folder_path,
            path_file_ids: HashMap::new(),
            save_logs: bot_settings.save_logs,
        }
    }

    pub fn log_image(&mut self, folder_name: &str, file_name: &str, img: &Image) {
        if !self.save_logs {
            return;
        }

        let file_path = self.get_path(folder_name, file_name, "png");
        img.save_image(Path::new(&file_path)).unwrap();
    }

    pub fn save_logs(&self) -> bool {
        self.save_logs
    }

    fn get_path(&mut self, folder_name: &str, file_name: &str, file_extension: &str) -> PathBuf {
        let folder_path = self.get_folder_path(folder_name);
        let folder_path_file_name = folder_path.join(file_name);

        fs::create_dir_all(&folder_path).unwrap(); // TODO Optimize. Don't do this everytime

        let file_id = self.path_file_ids.entry(folder_path_file_name).or_insert(0);

        let file_path = folder_path.join(Self::get_file_name(file_name, *file_id, file_extension));

        *file_id += 1;

        file_path
    }

    fn get_file_name(name: &str, id: u32, extension: &str) -> String {
        format!("{name}_{id}.{extension}")
    }

    fn get_folder_path(&self, folder_name: &str) -> PathBuf {
        self.log_folder_path.join(folder_name)
    }
}
