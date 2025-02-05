use super::{dc6_raw_bytes::Dc6RawBytes, dc6_scene::Dc6Scene};

pub struct Dc6StashRawBytes {
    dc6_raw_bytes: Dc6RawBytes,
}

impl Dc6StashRawBytes {
    pub fn new(dc6_raw_bytes: Dc6RawBytes) -> Self {
        Self { dc6_raw_bytes }
    }

    pub fn parse(&self) -> Dc6Scene {
        let dc6 = self.dc6_raw_bytes.parse();

        Dc6Scene {
            top_left: dc6.directions[0].encoded_frames[0].decode(),
            top_right: dc6.directions[0].encoded_frames[1].decode(),
            bottom_left: dc6.directions[0].encoded_frames[2].decode(),
            bottom_right: dc6.directions[0].encoded_frames[3].decode(),
        }
    }
}
