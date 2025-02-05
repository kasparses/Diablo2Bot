use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs};

const OUTPUT_FOLDER: &str = "Diablo2bot";
const SETTINGS_FOLDER: &str = "settings";
const EXECUTABLE_FILE: &str = "diablo2bot";
const EXECUTABLE_FILE_EXE: &str = "diablo2bot.exe";
const LICENSE_FILE: &str = "LICENSE";
const README_FILE: &str = "README.md";

fn main() {
    move_to_root_directory();
    build_application();
    remove_output_directory();
    create_output_directory();
    copy_license_to_output_directory();
    copy_readme_to_output_directory();
    copy_executable_to_output_directory();
    copy_settings_directory_to_output_directory();

    println!("Build and copy completed successfully!");
}

fn move_to_root_directory() {
    let project_path = get_project_path();
    std::env::set_current_dir(project_path).unwrap();
}

fn get_project_path() -> String {
    let mut path = env::current_dir().unwrap();

    // Continue to loop up the directory tree until we find the project path
    loop {
        let mut path_ = path.clone();
        path_.push("settings");
        if path_.exists() {
            break;
        }

        let success = path.pop();
        if !success {
            panic!("Could not find project path");
        }
    }

    path.to_str().unwrap().to_string()
}

fn build_application() {
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to execute cargo build --release");

    if !build_status.success() {
        eprintln!("Build failed");
        exit(1);
    }
}

fn remove_output_directory() {
    let output_dir = Path::new(OUTPUT_FOLDER);

    if output_dir.exists() {
        fs::remove_dir_all(output_dir).unwrap();
    }
}

fn create_output_directory() {
    let output_dir = Path::new(OUTPUT_FOLDER);

    if !output_dir.exists() {
        fs::create_dir(output_dir).unwrap();
    }
}

fn copy_license_to_output_directory() {
    let src = Path::new(LICENSE_FILE);
    let dst = Path::new(OUTPUT_FOLDER).join(LICENSE_FILE);

    fs::copy(src, dst).unwrap();
}

fn copy_readme_to_output_directory() {
    let src = Path::new(README_FILE);
    let dst = Path::new(OUTPUT_FOLDER).join(README_FILE);

    fs::copy(src, dst).unwrap();
}

fn get_executable_file_source() -> PathBuf {
    let release_folder_path = Path::new("target").join("release");

    let file_path = release_folder_path.join(EXECUTABLE_FILE);
    if file_path.exists() {
        file_path
    } else {
        release_folder_path.join(EXECUTABLE_FILE_EXE)
    }
}

fn copy_executable_to_output_directory() {
    let executable_file_source = get_executable_file_source();

    let executable_file_destination = Path::new(OUTPUT_FOLDER).join(EXECUTABLE_FILE_EXE);

    fs::copy(executable_file_source, executable_file_destination).unwrap();
}

fn copy_settings_directory_to_output_directory() {
    let settings_dir_source = Path::new(SETTINGS_FOLDER);
    let settings_dir_destination = Path::new(OUTPUT_FOLDER).join(SETTINGS_FOLDER);

    fs::create_dir(&settings_dir_destination).unwrap();

    copy_dir_recursive(settings_dir_source, &settings_dir_destination).unwrap();
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir(&dest_path)?;
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
