use std::{cmp::min, collections::HashSet};

use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};

use crate::{
    bot_settings::BotSettings,
    box_u16::BoxU16,
    constants::game_window_points::SAFE_POINT,
    point_u16::PointU16,
    units::Frames,
    utils::{sleep_frame, sleep_frames},
    ClickType,
};

pub struct OutputController {
    current_mouse_point: PointU16,
    held_keys: HashSet<Key>,
    enigo: Enigo,
    window_offset: PointU16,
    window_size: PointU16,
    bot_settings: BotSettings,
}

impl OutputController {
    pub fn new(
        enigo: Enigo,
        window_offset: PointU16,
        window_size: PointU16,
        current_mouse_point: PointU16,
        bot_settings: BotSettings,
    ) -> Self {
        Self {
            current_mouse_point,
            held_keys: HashSet::new(),
            enigo,
            window_offset,
            window_size,
            bot_settings,
        }
    }

    pub fn enter_command(&mut self, command: &str) {
        self.open_command_window();

        self.type_command(command);

        self.close_command_window();
    }

    fn open_command_window(&mut self) {
        self.press_key(enigo::Key::Return);

        sleep_frames(Frames(4));
    }

    fn type_forward_slash(&mut self) {
        #[cfg(target_os = "windows")]
        {
            self.press_key(enigo::Key::Divide);
        }

        #[cfg(not(target_os = "windows"))]
        {
            self.click_key(Key::Layout('/'));
        }
    }

    fn type_command(&mut self, command: &str) {
        self.type_forward_slash();
        sleep_frame();

        for c in command.chars() {
            self.click_key(Key::Layout(c));
        }

        sleep_frame();
    }

    fn close_command_window(&mut self) {
        self.press_key(enigo::Key::Return);

        sleep_frame();
        sleep_frame();
    }

    fn press_key(&mut self, key: Key) {
        self.enigo.key_click(key);
    }

    pub fn click_key(&mut self, key: Key) {
        self.enigo.key_click(key);
    }

    pub fn hold_key(&mut self, key: Key) {
        self.held_keys.insert(key);
        self.enigo.key_down(key);
    }

    pub fn release_key(&mut self, key: Key) {
        self.enigo.key_up(key);

        if self.held_keys.contains(&key) {
            self.held_keys.remove(&key);
            sleep_frames(self.bot_settings.num_frames_to_sleep_after_lifting_held_key);
        }
    }

    pub fn click_mouse(
        &mut self,
        point: PointU16,
        click_type: ClickType,
        sleep_after_cursor_movement: bool,
        sleep_after_click: bool,
    ) {
        self.move_mouse(point);

        if sleep_after_cursor_movement {
            sleep_frame();
            sleep_frame();
        }

        match click_type {
            ClickType::Left => self.enigo.mouse_click(enigo::MouseButton::Left),
            ClickType::Right => self.enigo.mouse_click(enigo::MouseButton::Right),
        }

        if sleep_after_click {
            sleep_frame();
        }
    }

    pub fn double_click(
        &mut self,
        point: PointU16,
        click_type: ClickType,
        sleep_after_cursor_movement: bool,
        sleep_after_click: bool,
    ) {
        self.click_mouse(point, click_type, sleep_after_cursor_movement, false);
        self.click_mouse(point, click_type, false, sleep_after_click);
    }

    pub fn move_mouse(&mut self, point: PointU16) {
        let point = self.truncate_point(point);
        self.current_mouse_point = point;

        let point = self.get_full_screen_point(point);
        self._move_mouse(point)
    }

    pub fn move_mouse_to_safe_point(&mut self) {
        if self.current_mouse_point != SAFE_POINT {
            self.move_mouse(SAFE_POINT);
            sleep_frame();
        }
    }

    pub fn ensure_mouse_is_out_of_area(&mut self, area: BoxU16) {
        if self.current_mouse_point.is_in_area(area) {
            self.move_mouse_to_safe_point();
        }
    }

    fn _move_mouse(&mut self, point: PointU16) {
        self.enigo
            .mouse_move_to(i32::from(point.col), i32::from(point.row));
    }

    fn truncate_point(&self, point: PointU16) -> PointU16 {
        PointU16 {
            row: min(self.window_size.row - 1, point.row),
            col: min(self.window_size.col - 1, point.col),
        }
    }

    fn get_full_screen_point(&self, point: PointU16) -> PointU16 {
        self.window_offset + point
    }
}

pub fn map_key_code(key_name: &str) -> enigo::Key {
    if key_name.len() == 1 {
        return enigo::Key::Layout(key_name.chars().next().unwrap());
    };

    match key_name.to_lowercase().to_string().as_str() {
        "alt" => enigo::Key::Alt,
        "backspace" => enigo::Key::Backspace,
        "caps_lock" => enigo::Key::CapsLock,
        "control" => enigo::Key::Control,
        "delete" => enigo::Key::Delete,
        "left_arrow" => enigo::Key::LeftArrow,
        "up_arrow" => enigo::Key::UpArrow,
        "right_arrow" => enigo::Key::RightArrow,
        "down_arrow" => enigo::Key::DownArrow,
        "end" => enigo::Key::End,
        "enter" => enigo::Key::Return,
        "esc" => enigo::Key::Escape,
        "f1" => enigo::Key::F1,
        "f2" => enigo::Key::F2,
        "f3" => enigo::Key::F3,
        "f4" => enigo::Key::F4,
        "f5" => enigo::Key::F5,
        "f6" => enigo::Key::F6,
        "f7" => enigo::Key::F7,
        "f8" => enigo::Key::F8,
        "f9" => enigo::Key::F9,
        "f10" => enigo::Key::F10,
        "f11" => enigo::Key::F11,
        "f12" => enigo::Key::F12,
        "home" => enigo::Key::Home,
        "page_down" => enigo::Key::PageDown,
        "page_up" => enigo::Key::PageUp,
        "shift" => enigo::Key::Shift,
        "space" => enigo::Key::Space,
        "tab" => enigo::Key::Tab,
        _ => panic!("Unknown key: {key_name}"),
    }
}
