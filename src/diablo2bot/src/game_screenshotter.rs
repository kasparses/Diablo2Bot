use crate::{image::Image, point_u16::PointU16, screenshotter::Screenshotter};

pub struct GameScreenshotter {
    screenshotter: Screenshotter,
    window_offset: PointU16,
    window_size: PointU16,
}

impl GameScreenshotter {
    pub fn new(
        screenshotter: Screenshotter,
        window_offset: PointU16,
        window_size: PointU16,
    ) -> Self {
        Self {
            screenshotter,
            window_offset,
            window_size,
        }
    }

    pub fn take_screenshot(&mut self) -> Image {
        self.screenshotter
            .take_screenshot_of_area(self.window_size, self.window_offset)
    }
}
