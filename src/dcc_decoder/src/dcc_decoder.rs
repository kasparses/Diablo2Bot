use crate::{
    bit_stream::BitStream,
    bit_streams::{BitStreamSizes, BitStreams},
    frame::{Frame, PointU16},
};

const FILE_SIGNATURE: u8 = 116;
const DEFAULT_CELL_SIZE: u8 = 4;
const BITS_WIDTH_TABLE: [u8; 16] = [0, 1, 2, 4, 6, 8, 10, 12, 14, 16, 20, 24, 26, 28, 30, 32];
const NUM_PIXEL_TABLE: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

#[derive(Debug)]
pub struct Dcc {
    pub directions: Vec<Direction>,
}

#[derive(Debug, Clone)]
struct PixelBufferEntry {
    pixels: [u8; 4],
    frame_idx: u16,
    frame_cell_idx: u16,
}

#[derive(Debug)]
struct DccHeader {
    num_frames_per_direction: u8,
    dir_offsets: Vec<u32>,
}

#[derive(Debug)]
pub struct Direction {
    pub frames: Vec<Frame>,
}

#[derive(Debug)]
struct DirectionHeader {
    compression_flag: u32,
    variable0_bits: u32,
    width_bits: u32,
    height_bits: u32,
    row_offset_bits: u32,
    col_offset_bits: u32,
    optional_data_bits: u32,
    coded_bytes_bits: u32,
}

#[derive(Debug)]
struct FrameHeader {
    height: u32,
    width: u32,
    row_offset: i32,
    col_offset: i32,
    optional_data: u32,
    frame_buttom_up: bool,
}

#[derive(Debug)]
struct DccBox {
    row_min: i16,
    col_min: i16,
    row_max: i16,
    col_max: i16,
    dims: PointU16,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub size: PointU16,
    pub offset: PointU16,
}

impl Cell {
    fn new(height: u16, width: u16, row_offset: u16, col_offset: u16) -> Self {
        Self {
            size: PointU16 {
                row: height,
                col: width,
            },
            offset: PointU16 {
                row: row_offset,
                col: col_offset,
            },
        }
    }

    fn default() -> Self {
        Self {
            size: PointU16 { row: 0, col: 0 },
            offset: PointU16 { row: 0, col: 0 },
        }
    }
}

#[derive(Debug)]
struct CellMatrix {
    num_vertical_cells: u16,
    num_horizontal_cells: u16,
    cells: Vec<Cell>,
}

impl<'dcc_file> Dcc {
    pub fn new(data: &'dcc_file [u8]) -> Self {
        let mut bs = BitStream::new(data);

        let header = DccHeader::load(&mut bs);
        let directions = Self::load_directions(&mut bs, &header);

        Self { directions }
    }

    fn load_directions(bs: &mut BitStream<'dcc_file>, header: &DccHeader) -> Vec<Direction> {
        let mut directions = Vec::new();

        for dir_offset in &header.dir_offsets {
            let mut direction_bit_stream = BitStream::new(&bs.bytes[*dir_offset as usize..]);

            let direction =
                Direction::load(&mut direction_bit_stream, header.num_frames_per_direction);
            directions.push(direction);
        }

        directions
    }
}

impl DccHeader {
    fn load(bs: &mut BitStream) -> Self {
        let file_signature = bs.stream_alligned_byte();

        if file_signature != FILE_SIGNATURE {
            panic!("Incorrect signature");
        }

        let _version = bs.stream_alligned_byte();

        let num_directions = bs.stream_alligned_byte();
        let num_frames_per_direction = bs.stream_alligned_uint() as u8;
        let tag = bs.stream_alligned_uint();

        let _total_size = bs.stream_alligned_uint();

        assert_eq!(tag, 1);

        let dir_offsets: Vec<u32> = (0..num_directions)
            .map(|_| bs.stream_alligned_uint())
            .collect();

        Self {
            num_frames_per_direction,
            dir_offsets,
        }
    }
}

impl Direction {
    fn load(bs: &mut BitStream<'_>, num_frames_per_direction: u8) -> Self {
        let direction_header = DirectionHeader::load(bs);

        let frame_headers: Vec<FrameHeader> = (0..num_frames_per_direction)
            .map(|_| FrameHeader::load(bs, &direction_header))
            .collect();
        let frame_boxes: Vec<DccBox> = frame_headers.iter().map(get_frame_box).collect();

        let _optional_bytes = Self::get_optional_bytes(bs, &frame_headers);

        let direction_box = Self::get_direction_box(&frame_boxes);
        let bit_stream_sizes = BitStreamSizes::load(bs, direction_header.compression_flag);
        let pixel_values = Self::get_dcc_pixel_values_key(bs);
        let mut bit_streams = BitStreams::load(bs, &bit_stream_sizes, bs.bytes);

        let (pixel_buffer_entries, mut buffer_cells, mut frames_cells) = Self::get_pixel_buffer(
            &mut bit_streams,
            &direction_box,
            &frame_boxes,
            &pixel_values,
        );

        let frame_bitmaps = Self::make_frames(
            &pixel_buffer_entries,
            &mut buffer_cells,
            &mut frames_cells,
            &direction_box,
            &mut bit_streams.pixel_code_and_displacment,
        );

        Self {
            frames: frame_bitmaps,
        }
    }

    fn make_frames(
        pixel_buffer_entries: &[PixelBufferEntry],
        buffer_cells: &mut CellMatrix,
        frames_cells: &mut [CellMatrix],
        dir_box: &DccBox,
        pixel_index_bitstream: &mut BitStream,
    ) -> Vec<Frame> {
        let mut frame_bitmaps = Vec::with_capacity(frames_cells.len());

        let mut dir_bitmap = Frame::new(dir_box.dims);

        let mut pbe_iter = pixel_buffer_entries.iter();
        let mut pbe = pbe_iter.next().unwrap().clone();

        for (f, frame_cells) in frames_cells.iter().enumerate() {
            let mut frame_bitmap = Frame::new(dir_bitmap.dims);
            for (c, frame_cell) in frame_cells.cells.iter().enumerate() {
                let cell_row = frame_cell.offset.row / 4;
                let cell_col = frame_cell.offset.col / 4;

                let cell_idx = ((cell_row * buffer_cells.num_horizontal_cells) + cell_col) as usize;
                let buff_cell = buffer_cells.cells.get_mut(cell_idx).unwrap();

                if (pbe.frame_idx == f as u16) && (pbe.frame_cell_idx == c as u16) {
                    if pbe.pixels[0] == pbe.pixels[1] {
                        dir_bitmap.set_cell_to_color(*frame_cell, pbe.pixels[0]);
                    } else {
                        let num_bits_per_pixel_index =
                            if pbe.pixels[1] == pbe.pixels[2] { 1 } else { 2 };

                        dir_bitmap.set_cell(*frame_cell, || {
                            pbe.pixels[pixel_index_bitstream.stream_byte(num_bits_per_pixel_index)
                                as usize]
                        });
                    }

                    dir_bitmap.copy_cell_external(&mut frame_bitmap, *frame_cell);

                    if let Some(p) = pbe_iter.next() {
                        pbe = p.clone()
                    }
                } else if frame_cell.size == buff_cell.size {
                    dir_bitmap.copy_cell_internal(
                        buff_cell.offset,
                        frame_cell.offset,
                        buff_cell.size,
                    );
                    dir_bitmap.copy_cell_external(&mut frame_bitmap, *frame_cell);
                } else {
                    dir_bitmap.set_cell_to_color(*frame_cell, 0);
                }

                *buff_cell = *frame_cell;
            }

            frame_bitmaps.push(frame_bitmap);
        }

        frame_bitmaps
    }

    fn get_pixel_buffer(
        bit_streams: &mut BitStreams,
        direction_box: &DccBox,
        frame_boxes: &[DccBox],
        pixel_values: &[u8; 256],
    ) -> (Vec<PixelBufferEntry>, CellMatrix, Vec<CellMatrix>) {
        let mut pixel_buffer_entries: Vec<PixelBufferEntry> = Vec::new();

        let buffer_cells = Self::get_buffer_cells(direction_box);

        let mut cell_buffer: Vec<i32> = vec![
            -1;
            buffer_cells.num_vertical_cells as usize
                * buffer_cells.num_horizontal_cells as usize
        ];

        let mut frames_cells: Vec<CellMatrix> = Vec::with_capacity(frame_boxes.len());

        for (f, frame_box) in frame_boxes.iter().enumerate() {
            let frame_cells = Self::get_frame_cells(direction_box, frame_box);

            let num_vertical_cells = frame_cells.num_vertical_cells;
            let num_horizontal_cells = frame_cells.num_horizontal_cells;

            frames_cells.push(frame_cells);

            let cell_row_offset = (frame_box.row_min - direction_box.row_min) / 4;
            let cell_col_offset = (frame_box.col_min - direction_box.col_min) / 4;

            for row in 0..num_vertical_cells {
                let current_row_cell = cell_row_offset + row as i16;
                for col in 0..num_horizontal_cells {
                    let current_col_cell = cell_col_offset + col as i16;
                    let current_cell_id = current_col_cell
                        + (current_row_cell * buffer_cells.num_horizontal_cells as i16);

                    let old_entry_id = cell_buffer[current_cell_id as usize];

                    if old_entry_id != -1 {
                        let next_cell = match &mut bit_streams.equal_cell {
                            Some(bs) => match bs.stream_bit() {
                                true => true,
                                false => false,
                            },
                            None => false,
                        };

                        if !next_cell {
                            let pixel_mask = bit_streams.pixel_mask.stream_byte(4);
                            let num_pixels = NUM_PIXEL_TABLE[pixel_mask as usize];

                            let (pixel_indices, num_decoded_pixels) =
                                stream_pixel_indices(num_pixels, bit_streams);

                            let mut curr_idx = num_decoded_pixels - 1;

                            let mut val = [0; 4];

                            for (i, value) in val.iter_mut().enumerate() {
                                if pixel_mask & (1 << i) != 0 {
                                    if curr_idx >= 0 {
                                        *value = pixel_indices[curr_idx as usize];
                                        curr_idx -= 1;
                                    }
                                } else {
                                    *value = pixel_buffer_entries[old_entry_id as usize].pixels[i];
                                }
                            }

                            cell_buffer[current_cell_id as usize] =
                                pixel_buffer_entries.len() as i32;
                            pixel_buffer_entries.push(PixelBufferEntry {
                                pixels: val,
                                frame_idx: f as u16,
                                frame_cell_idx: ((row * num_horizontal_cells) + col),
                            });
                        }
                    } else {
                        let (pixel_indices, num_decoded_pixels) =
                            stream_pixel_indices(4, bit_streams);

                        let mut curr_idx = num_decoded_pixels - 1;

                        let mut val = [0; 4];

                        for value in &mut val {
                            if curr_idx >= 0 {
                                *value = pixel_indices[curr_idx as usize];
                                curr_idx -= 1;
                            }
                        }

                        cell_buffer[current_cell_id as usize] = pixel_buffer_entries.len() as i32;
                        pixel_buffer_entries.push(PixelBufferEntry {
                            pixels: val,
                            frame_idx: f as u16,
                            frame_cell_idx: ((row * num_horizontal_cells) + col),
                        });
                    };
                }
            }
        }

        for pb in &mut pixel_buffer_entries {
            for i in 0..4 {
                let y = pb.pixels[i];
                pb.pixels[i] = pixel_values[y as usize];
            }
        }

        (pixel_buffer_entries, buffer_cells, frames_cells)
    }

    fn get_frame_cells(dir_box: &DccBox, frame_box: &DccBox) -> CellMatrix {
        let cells_height =
            Self::get_frame_cell_sizes(frame_box.dims.row, frame_box.row_min, dir_box.row_min);
        let cells_width =
            Self::get_frame_cell_sizes(frame_box.dims.col, frame_box.col_min, dir_box.col_min);

        let mut cells: Vec<Cell> = Vec::with_capacity(cells_width.len() * cells_height.len());

        let mut row_offset = frame_box.row_min - dir_box.row_min;
        for cell_height in &cells_height {
            let mut col_offset = frame_box.col_min - dir_box.col_min;
            for cell_width in &cells_width {
                cells.push(Cell::new(
                    *cell_height,
                    *cell_width,
                    row_offset as u16,
                    col_offset as u16,
                ));

                col_offset += *cell_width as i16;
            }
            row_offset += *cell_height as i16;
        }

        CellMatrix {
            cells,
            num_vertical_cells: cells_height.len() as u16,
            num_horizontal_cells: cells_width.len() as u16,
        }
    }

    fn get_frame_cell_sizes(frame_size: u16, frame_min: i16, dir_min: i16) -> Vec<u16> {
        let first_dimension_cell_pixel_size = (i16::from(DEFAULT_CELL_SIZE)
            - ((frame_min - dir_min) % i16::from(DEFAULT_CELL_SIZE)))
            as u8;
        let num_cells = Self::get_num_frame_cells(frame_size, first_dimension_cell_pixel_size);

        let mut cell_sizes: Vec<u16> = Vec::with_capacity(num_cells as usize);
        if num_cells == 1 {
            cell_sizes.push(frame_size)
        } else {
            cell_sizes.push(u16::from(first_dimension_cell_pixel_size));
            for _ in 1..(num_cells - 1) {
                cell_sizes.push(u16::from(DEFAULT_CELL_SIZE))
            }
            cell_sizes.push(
                frame_size
                    - u16::from(first_dimension_cell_pixel_size)
                    - (u16::from(DEFAULT_CELL_SIZE) * (num_cells - 2)),
            );
        }

        cell_sizes
    }

    fn get_num_frame_cells(frame_size: u16, first_dimension_cell_pixel_size: u8) -> u16 {
        if (frame_size as i16 - i16::from(first_dimension_cell_pixel_size)) <= 1 {
            1
        } else {
            let tmp = frame_size - u16::from(first_dimension_cell_pixel_size) - 1;
            let mut num_frame_cells = 2 + (tmp / u16::from(DEFAULT_CELL_SIZE));
            if tmp % u16::from(DEFAULT_CELL_SIZE) == 0 {
                num_frame_cells -= 1;
            }
            num_frame_cells
        }
    }

    fn get_buffer_cells(direction_box: &DccBox) -> CellMatrix {
        let num_vertical_cells =
            1 + ((direction_box.dims.row as i16 - 1) / i16::from(DEFAULT_CELL_SIZE)) as u16;
        let num_horizontal_cells =
            1 + ((direction_box.dims.col as i16 - 1) / i16::from(DEFAULT_CELL_SIZE)) as u16;

        let mut cells =
            Vec::with_capacity(num_vertical_cells as usize * num_horizontal_cells as usize);
        for _ in 0..num_vertical_cells {
            for _ in 0..num_horizontal_cells {
                cells.push(Cell::default());
            }
        }

        CellMatrix {
            cells,
            num_vertical_cells,
            num_horizontal_cells,
        }
    }

    fn get_dcc_pixel_values_key(bs: &mut BitStream) -> [u8; 256] {
        let mut pixel_values = [0; 256];

        let mut c = 0;
        for i in 0..256 {
            let has_pixel = bs.stream_bit();
            if has_pixel {
                pixel_values[c] = i as u8;
                c += 1;
            }
        }

        pixel_values
    }

    fn get_optional_bytes(bs: &mut BitStream, frame_headers: &Vec<FrameHeader>) -> Vec<Vec<u8>> {
        let mut has_optional_data = false;
        for frame_header in frame_headers {
            if frame_header.optional_data > 0 {
                has_optional_data = true;
                break;
            }
        }

        if !has_optional_data {
            return Vec::new();
        }

        if bs.bit_offset != 0 {
            bs.bit_offset = 0;
            bs.byte_offset += 1;
        }

        frame_headers
            .iter()
            .map(|header| get_optional_bytes(bs, header))
            .collect()
    }

    fn get_direction_box(frame_boxes: &[DccBox]) -> DccBox {
        let row_min = frame_boxes
            .iter()
            .map(|frame_box| frame_box.row_min)
            .min()
            .unwrap();
        let col_min = frame_boxes
            .iter()
            .map(|frame_box| frame_box.col_min)
            .min()
            .unwrap();
        let row_max = frame_boxes
            .iter()
            .map(|frame_box| frame_box.row_max)
            .max()
            .unwrap();
        let col_max = frame_boxes
            .iter()
            .map(|frame_box| frame_box.col_max)
            .max()
            .unwrap();

        let height = (row_max - row_min + 1) as u16;
        let width = (col_max - col_min + 1) as u16;
        let dims = PointU16 {
            row: height,
            col: width,
        };

        DccBox {
            row_min,
            col_min,
            row_max,
            col_max,
            dims,
        }
    }
}

fn stream_pixel_indices(num_pixels: u8, bit_streams: &mut BitStreams) -> ([u8; 4], i32) {
    let (pixel_indices, num_decoded_pixels) = if num_pixels != 0 {
        match &mut bit_streams.encoding_type_raw_pixel {
            Some(encoding_type_raw_pixel_bitstream) => {
                let is_raw_encoding = encoding_type_raw_pixel_bitstream.encoding_type.stream_bit();
                if is_raw_encoding {
                    stream_pixels_indices_raw_encoding(
                        num_pixels,
                        &mut encoding_type_raw_pixel_bitstream.raw_pixel,
                    )
                } else {
                    stream_pixels_indices_encoded(
                        num_pixels,
                        &mut bit_streams.pixel_code_and_displacment,
                    )
                }
            }
            None => stream_pixels_indices_encoded(
                num_pixels,
                &mut bit_streams.pixel_code_and_displacment,
            ),
        }
    } else {
        ([0; 4], 0)
    };
    (pixel_indices, num_decoded_pixels)
}

fn stream_pixels_indices_raw_encoding(num_pixels: u8, bs: &mut BitStream) -> ([u8; 4], i32) {
    let mut indices = [0; 4];

    let mut last_idx = 0;
    let mut c = 0;

    for idx in indices.iter_mut().take(num_pixels as usize) {
        *idx = bs.stream_byte(8);

        if *idx == last_idx {
            *idx = 0;
            break;
        } else {
            last_idx = *idx;
            c += 1;
        }
    }

    (indices, c)
}

fn stream_pixels_indices_encoded(num_pixels: u8, bs: &mut BitStream) -> ([u8; 4], i32) {
    let mut indices = [0; 4];

    let mut last_idx = 0;
    let mut c = 0;

    for idx in indices.iter_mut().take(num_pixels as usize) {
        *idx = last_idx;

        let mut displ = bs.stream_byte(4);
        *idx += displ;

        while displ == 15 {
            displ = bs.stream_byte(4);
            *idx += displ;
        }

        if *idx == last_idx {
            *idx = 0;

            break;
        } else {
            last_idx = *idx;
            c += 1;
        }
    }

    (indices, c)
}

impl DirectionHeader {
    fn load(bs: &mut BitStream) -> Self {
        let _outsize_coded = bs.stream_bits(32);

        let compression_flag = u32::from(bs.stream_byte(2));
        let variable0_bits = u32::from(bs.stream_byte(4));
        let width_bits = u32::from(bs.stream_byte(4));
        let height_bits = u32::from(bs.stream_byte(4));
        let col_offset_bits = u32::from(bs.stream_byte(4));
        let row_offset_bits = u32::from(bs.stream_byte(4));
        let optional_data_bits = u32::from(bs.stream_byte(4));
        let coded_bytes_bits = u32::from(bs.stream_byte(4));

        Self {
            compression_flag,
            variable0_bits,
            width_bits,
            height_bits,
            col_offset_bits,
            row_offset_bits,
            optional_data_bits,
            coded_bytes_bits,
        }
    }
}

fn get_optional_bytes(bs: &mut BitStream, header: &FrameHeader) -> Vec<u8> {
    let mut optional_bytes = Vec::new();

    for _ in 0..header.optional_data {
        let byte = bs.stream_alligned_byte();
        optional_bytes.push(byte);
    }

    optional_bytes
}

fn get_frame_box(header: &FrameHeader) -> DccBox {
    let (row_min, row_max) = if header.frame_buttom_up {
        let row_min = header.row_offset as i16;
        let row_max = row_min + (header.height as i16) - 1;
        (row_min, row_max)
    } else {
        let row_max = header.row_offset as i16;
        let row_min = row_max - (header.height as i16) + 1;
        (row_min, row_max)
    };

    let col_min = header.col_offset as i16;
    let col_max = col_min + (header.width as i16) - 1;

    let height = (row_max - row_min + 1) as u16;
    let width = (col_max - col_min + 1) as u16;
    let dims = PointU16 {
        row: height,
        col: width,
    };

    DccBox {
        row_min,
        col_min,
        row_max,
        col_max,
        dims,
    }
}

impl FrameHeader {
    fn load(bs: &mut BitStream, direction_header: &DirectionHeader) -> Self {
        let _variable0 = bs.stream_bits(BITS_WIDTH_TABLE[direction_header.variable0_bits as usize]);

        let width = bs.stream_bits(BITS_WIDTH_TABLE[direction_header.width_bits as usize]);
        let height = bs.stream_bits(BITS_WIDTH_TABLE[direction_header.height_bits as usize]);
        let col_offset =
            bs.stream_signed_bits(BITS_WIDTH_TABLE[direction_header.col_offset_bits as usize]);
        let row_offset =
            bs.stream_signed_bits(BITS_WIDTH_TABLE[direction_header.row_offset_bits as usize]);
        let optional_data =
            bs.stream_bits(BITS_WIDTH_TABLE[direction_header.optional_data_bits as usize]);

        let _coded_bytes =
            bs.stream_bits(BITS_WIDTH_TABLE[direction_header.coded_bytes_bits as usize]);

        let frame_buttom_up = bs.stream_bit();

        Self {
            height,
            width,
            row_offset,
            col_offset,
            optional_data,
            frame_buttom_up,
        }
    }
}
