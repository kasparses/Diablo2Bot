use std::{thread, time::Duration};

use crate::units::{Frames, Milliseconds};

pub fn sleep_millis(duration: Milliseconds) {
    thread::sleep(Duration::from_millis(duration.0));
}

pub fn sleep_frame() {
    sleep_millis(Milliseconds::from(Frames(1)));
}

pub fn sleep_frames(frames: Frames) {
    sleep_millis(Milliseconds::from(frames));
}
