#[cfg(test)]
pub mod test_utils {
    use std::{
        fs::{self, File},
        io::{self, BufReader},
        path::Path,
    };

    use serde::de::DeserializeOwned;

    use crate::{
        enums::act::Act, file_io::FileIo, image::Image, point_u16::PointU16, structs::Pixel,
        table::Table,
    };

    const TEMP: &str = "temp";

    #[derive(Debug)]
    pub struct FileData {
        pub name: String,
        pub path: String,
    }

    #[derive(Debug)]
    pub struct Directory {
        pub name: String,
        pub path: String,
        pub subdirectories: Vec<Directory>,
        pub files: Vec<FileData>,
    }

    pub fn read_json<T: DeserializeOwned>(file_path: &Path) -> std::io::Result<T> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }

    pub fn get_directory(parent_dir_path: &str, dir_name: &str) -> Directory {
        let mut subdirectories = Vec::new();
        let mut files = Vec::new();

        let dir_path = format!("{}/{}", parent_dir_path, dir_name);

        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
                            subdirectories.push(get_directory(&dir_path, file_name));
                        }
                        if entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false) {
                            files.push(FileData {
                                name: file_name.to_string(),
                                path: entry.path().to_str().unwrap().to_string(),
                            });
                        }
                    }
                }
            }
        }

        subdirectories.sort_by(|a, b| a.name.cmp(&b.name));
        files.sort_by(|a, b| a.name.cmp(&b.name));

        Directory {
            name: dir_name.to_string(),
            path: dir_path.to_string(),
            subdirectories,
            files,
        }
    }

    impl Image {
        pub fn load_image(path: &Path) -> Self {
            let decoder = png::Decoder::new(File::open(path).unwrap());
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).unwrap();

            let dims = PointU16 {
                row: info.height as u16,
                col: info.width as u16,
            };

            let bytes = &buf[..info.buffer_size()];

            let size: usize = match info.color_type {
                png::ColorType::Rgb => 3,
                png::ColorType::Rgba => 4,
                _ => todo!(),
            };

            let pixels: Vec<Pixel> = bytes
                .chunks(size)
                .map(|x| Pixel {
                    red: x[0],
                    green: x[1],
                    blue: x[2],
                })
                .collect();

            Image { dims, pixels }
        }
    }

    impl Table {
        pub fn print(&self) {
            for row in self.cells.iter() {
                for cell in row.iter() {
                    match &cell {
                        Some(item) => {
                            print!("{: <10}", item);
                        }
                        None => {
                            print!("{: <10}", "None");
                        }
                    }
                }
                println!();
            }
        }
    }

    impl Act {
        pub fn from_str(name: &str) -> Self {
            match name {
                "ACT1" => Self::Act1,
                "ACT2" => Self::Act2,
                "ACT3" => Self::Act3,
                "ACT4" => Self::Act4,
                "ACT5" => Self::Act5,
                _ => {
                    panic!("Wrong act")
                }
            }
        }
    }

    impl FileIo {
        pub fn create_temp_dir(&self) -> io::Result<()> {
            fs::create_dir_all(self.root.join(TEMP))
        }
    }
}
