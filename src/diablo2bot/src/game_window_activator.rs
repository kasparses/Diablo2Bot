#[cfg(target_os = "windows")]
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, process::Command, ptr};

#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::HWND__,
    um::winuser::{FindWindowW, GetForegroundWindow, SetForegroundWindow},
};

#[cfg(target_os = "linux")]
use std::process::Command;

use crate::{
    bot_settings::BotSettings, constants::game_name::GAME_NAME,
    game_screenshotter::GameScreenshotter, get_game_window_location,
    output_controller::OutputController, spell_caster::get_current_time_milliseconds,
    utils::sleep_millis,
};

#[cfg(target_os = "linux")]
struct Window {
    name: String,
    id: String,
}

// GetClassNameW(window_handle, lpClassName, 1);
// if win32gui.GetClassName(game_window_handle) == "CabinetWClass":
//     print("You have a folder named 'Diablo II' open in file explorer. Please close this folder as the script cannot find the real Diablo 2 window otherwise")
//     sys.exit()

pub fn start_diablo2(
    bot_settings: &BotSettings,
    diablo2_folder_path: &str,
) -> (GameScreenshotter, OutputController) {
    // TODO Do this in another thread so I can load the other data while I am waiting for the game to start
    let mut is_started_game_from_script = false;

    if !is_game_started() {
        is_started_game_from_script = true;

        let diablo2_exe_file_path = &format!("{diablo2_folder_path}/{GAME_NAME}.exe");

        if let Err(e) = std::fs::File::open(diablo2_exe_file_path) {
            panic!("Could not open file {diablo2_exe_file_path} due to error: {e}");
        }

        start_game(diablo2_exe_file_path);

        let start_loop_time_milliseconds = get_current_time_milliseconds();

        while !is_game_started() {
            sleep_millis(
                bot_settings
                    .game_startup_settings
                    .check_game_started_cooldown_milliseconds,
            );
            if get_current_time_milliseconds() - start_loop_time_milliseconds
                > bot_settings
                    .game_startup_settings
                    .max_milliseconds_check_game_started
            {
                panic!("Could not start game!");
            }
        }
    }

    activate_game_window(bot_settings);

    get_game_window_location(bot_settings, is_started_game_from_script)
}

fn start_game(diablo2_exe_file_path: &str) {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", diablo2_exe_file_path, "-w"])
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("wine")
            .args([diablo2_exe_file_path, "-w"])
            .spawn()
            .unwrap();
    }
}

fn is_game_started() -> bool {
    #[cfg(target_os = "windows")]
    {
        get_window_handle(GAME_NAME).is_some()
    }

    #[cfg(not(target_os = "windows"))]
    {
        let windows = get_active_windows();

        for window in windows {
            if window.name == GAME_NAME {
                return true;
            }
        }

        false
    }
}

#[cfg(target_os = "linux")]
fn line_to_window(line: &str) -> Window {
    let columns: Vec<&str> = line.split_whitespace().collect();
    let id = columns[0].to_owned();
    let name = columns[3..].join(" ");

    Window { name, id }
}

#[cfg(target_os = "linux")]
fn get_active_windows() -> Vec<Window> {
    // TODO xdotool search --limit 1 --name 'Diablo II'
    let output = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    result.lines().map(line_to_window).collect()
}

#[cfg(target_os = "linux")]
fn activate_window(id: &str) {
    Command::new("xdotool")
        .arg("windowactivate")
        .arg(id)
        .spawn()
        .unwrap();
}

#[cfg(target_os = "windows")]
fn get_window_handle(window_name: &str) -> Option<*mut winapi::shared::windef::HWND__> {
    let window_name_wide: Vec<u16> = OsStr::new(window_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let window_class_name = ptr::null();
    let window_handle: *mut winapi::shared::windef::HWND__ =
        unsafe { FindWindowW(window_class_name, window_name_wide.as_ptr()) };

    if window_handle.is_null() {
        return None;
    }

    Some(window_handle)
}

fn activate_game_window(_bot_settings: &BotSettings) {
    #[cfg(target_os = "windows")]
    {
        let game_window_handle = get_window_handle(GAME_NAME).unwrap();

        if !is_game_window_active(game_window_handle) {
            unsafe {
                SetForegroundWindow(game_window_handle);

                let start_loop_time_milliseconds = get_current_time_milliseconds();

                while !is_game_window_active(game_window_handle) {
                    sleep_millis(
                        _bot_settings
                            .game_startup_settings
                            .check_game_started_cooldown_milliseconds,
                    );
                    if get_current_time_milliseconds() - start_loop_time_milliseconds
                        > _bot_settings
                            .game_startup_settings
                            .max_milliseconds_check_game_started
                    {
                        panic!("Could not activate game window");
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let window = get_window_with_name(GAME_NAME);

        if let Some(window) = window {
            activate_window(&window.id);
        }
    }
}

#[cfg(target_os = "windows")]
fn is_game_window_active(game_window_handle: *mut HWND__) -> bool {
    let foreground_window_handle = unsafe { GetForegroundWindow() };
    foreground_window_handle == game_window_handle
}

#[cfg(target_os = "linux")]
fn get_window_with_name(name: &str) -> Option<Window> {
    get_active_windows()
        .into_iter()
        .find(|window| window.name == name)
}
