use crate::byte_stream::ByteStream;

const END_OF_SCAN_LINE: u8 = 128;
const MAX_RUN_LENGTH: u8 = 127;
const EXPECTED_DC6_VERSION: i32 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanlineState {
    EndOfLine,
    RunOfTransparentPixels,
    RunOfOpaquePixels,
}

pub struct Dc6<'dc6_file> {
    pub directions: Vec<Direction<'dc6_file>>,
}

pub struct Direction<'dc6_file> {
    pub encoded_frames: Vec<EncodedFrame<'dc6_file>>,
}

#[derive(Clone, Copy)]
pub struct FrameMetadata {
    pub width: u32,
    pub height: u32,
    pub offset_row: i32,
    pub offset_col: i32,
}

pub struct EncodedFrame<'dc6_file> {
    pub meta_data: FrameMetadata,
    encoded_bytes: &'dc6_file [u8],
}

pub struct DecodedFrame {
    pub meta_data: FrameMetadata,
    pub decoded_bytes: Vec<u8>,
}

struct Dc6Header {
    num_directions: u32,
    num_frames_per_direction: u32,
}

impl<'dc6_file> Dc6<'dc6_file> {
    pub fn new(data: &'dc6_file [u8]) -> Self {
        let mut byte_reader = ByteStream::new(data);
        let header = Dc6Header::load(&mut byte_reader);

        Self {
            directions: Self::load_directions(&mut byte_reader, &header),
        }
    }

    fn load_directions(
        byte_reader: &mut ByteStream<'dc6_file>,
        header: &Dc6Header,
    ) -> Vec<Direction<'dc6_file>> {
        (0..header.num_directions)
            .map(|_| Direction::load(byte_reader, header.num_frames_per_direction))
            .collect()
    }
}

impl Dc6Header {
    fn load(byte_reader: &mut ByteStream) -> Self {
        let version = byte_reader.stream_int();
        if version != EXPECTED_DC6_VERSION {
            panic!("unexpected dc6 version")
        }

        // Skip flags, encoding and termination
        byte_reader.skip_bytes(4 * 3);

        let num_directions = byte_reader.stream_uint();
        let num_frames_per_direction = byte_reader.stream_uint();

        // Skip blocks
        byte_reader.skip_bytes(num_directions * num_frames_per_direction * 4);

        Self {
            num_directions,
            num_frames_per_direction,
        }
    }
}

impl<'dc6_file> Direction<'dc6_file> {
    fn load(byte_reader: &mut ByteStream<'dc6_file>, num_frames_per_direction: u32) -> Self {
        let frames = (0..num_frames_per_direction)
            .map(|_| EncodedFrame::load(byte_reader))
            .collect();
        Self {
            encoded_frames: frames,
        }
    }
}

impl<'dc6_file> EncodedFrame<'dc6_file> {
    fn load(byte_reader: &mut ByteStream<'dc6_file>) -> Self {
        // Skip flipped
        byte_reader.skip_bytes(4);

        let width = byte_reader.stream_uint();
        let height = byte_reader.stream_uint();

        let offset_row = byte_reader.stream_int();
        let offset_col = byte_reader.stream_int();

        let meta_data = FrameMetadata {
            height,
            width,
            offset_row,
            offset_col,
        };

        // unknown, next_block
        byte_reader.skip_bytes(2 * 4);

        let num_frame_bytes = byte_reader.stream_uint();
        let encoded_bytes = byte_reader.stream_bytes(num_frame_bytes);

        // Skip terminator
        byte_reader.skip_bytes(3);

        Self {
            meta_data,
            encoded_bytes,
        }
    }

    pub fn decode(&self) -> DecodedFrame {
        let mut data: Vec<u8> =
            vec![0; self.meta_data.width as usize * self.meta_data.height as usize];

        let mut col: u32 = 0;
        let mut row = self.meta_data.height - 1;

        let mut encoded_byte_reader = ByteStream::new(self.encoded_bytes);

        loop {
            let byte = encoded_byte_reader.stream_byte();

            let scan_line_type = Self::scan_line_type(byte);
            match scan_line_type {
                ScanlineState::EndOfLine => {
                    if row == 0 {
                        break;
                    }

                    row -= 1;
                    col = 0;
                }
                ScanlineState::RunOfTransparentPixels => {
                    let num_transparent_pixels = byte & MAX_RUN_LENGTH;
                    col += u32::from(num_transparent_pixels);
                }
                ScanlineState::RunOfOpaquePixels => {
                    let idx = row as usize * self.meta_data.width as usize + col as usize;
                    for i in 0..byte as usize {
                        data[idx + i] = encoded_byte_reader.stream_byte();
                    }

                    col += u32::from(byte);
                }
            }
        }

        DecodedFrame {
            meta_data: self.meta_data,
            decoded_bytes: data,
        }
    }

    fn scan_line_type(byte: u8) -> ScanlineState {
        if byte == END_OF_SCAN_LINE {
            return ScanlineState::EndOfLine;
        }

        if (byte & END_OF_SCAN_LINE) > 0 {
            return ScanlineState::RunOfTransparentPixels;
        }

        ScanlineState::RunOfOpaquePixels
    }
}
