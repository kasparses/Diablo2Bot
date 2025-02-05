use scrap::{Capturer, Display};
use std::{io::ErrorKind::WouldBlock, thread, time::Duration};

use crate::{image::Image, point_u16::PointU16, structs::Pixel};

pub struct Screenshotter {
    capturer: Capturer,
}

impl Screenshotter {
    pub fn new() -> Self {
        let display = Display::primary().expect("Couldn't find primary display.");
        let capturer = Capturer::new(display).expect("Couldn't begin capture.");

        Self { capturer }
    }

    pub fn take_full_screenshot(&mut self) -> Image {
        let frame = loop {
            match self.capturer.frame() {
                Ok(frame) => break frame,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    } else {
                        panic!("Error capturing screen: {error:?}");
                    }
                }
            }
        };

        let pixels = frame
            .chunks_exact(4)
            .map(|bgra| Pixel {
                blue: bgra[0],
                green: bgra[1],
                red: bgra[2],
            })
            .collect();

        let frame_dims = PointU16 {
            row: self.capturer.height() as u16,
            col: self.capturer.width() as u16,
        };

        Image {
            dims: frame_dims,
            pixels,
        }
    }

    pub fn take_screenshot_of_area(
        &mut self,
        window_size: PointU16,
        window_offset: PointU16,
    ) -> Image {
        let screen_width = self.capturer.width();

        let frame = loop {
            match self.capturer.frame() {
                Ok(frame) => break frame,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    } else {
                        panic!("Error capturing screen: {error:?}");
                    }
                }
            }
        };

        let mut pixels = Vec::with_capacity(window_size.len() as usize);

        let top = window_offset.row as usize;
        let bottom = top + window_size.row as usize;
        let left = window_offset.col as usize;
        let right = left + window_size.col as usize;

        for row in top..bottom {
            for col in left..right {
                let idx = (row * screen_width + col) * 4;
                let pixel = Pixel {
                    blue: frame[idx],
                    green: frame[idx + 1],
                    red: frame[idx + 2],
                };
                pixels.push(pixel);
            }
        }

        Image {
            dims: window_size,
            pixels,
        }
    }
}
