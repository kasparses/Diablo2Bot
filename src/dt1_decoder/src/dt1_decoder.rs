use crate::byte_stream::ByteStream;

#[derive(Debug)]
pub struct Dt1<'dt1_file> {
    pub header: Dt1Header,
    pub tiles: Vec<Tile<'dt1_file>>,
}

#[derive(Debug)]
pub struct Dt1Header {
    pub version1: u32,
    pub version2: u32,
    pub num_tiles: u32,
}

#[derive(Debug)]
pub struct Tile<'dt1_file> {
    pub header: TileHeader,
    pub sub_tiles: Vec<SubTile<'dt1_file>>,
}

#[derive(Debug)]
pub struct TileHeader {
    pub direction: u32,
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub unknown_bytes_1: [u8; 4],
    pub orientation: u32,
    pub main_index: u32,
    pub sub_index: u32,
    pub frame_index: u32,
    pub unknown_bytes_2: [u8; 4],
    pub sub_tiles_flags: [u8; 25],
    pub unknown_bytes_3: [u8; 7],
    pub tiles_ptr: u32,
    pub tiles_length: u32,
    pub num_tiles: u32,
    pub unknown_bytes_4: [u8; 12],
}

#[derive(Debug)]
pub struct SubTile<'dt1_file> {
    pub header: SubTileHeader,
    pub data: &'dt1_file [u8],
}

#[derive(Debug)]
pub struct SubTileHeader {
    pub x_position: u16,
    pub y_position: i16,
    pub unknown_bytes_1: [u8; 2],
    pub x_grid_position: u8,
    pub y_grid_position: u8,
    pub format: u16,
    pub data_length: u32,
    pub unknown_bytes_2: [u8; 2],
    pub data_offset: u32,
}

impl<'dt1_file> Dt1<'dt1_file> {
    pub fn new(data: &'dt1_file [u8]) -> Self {
        let mut byte_stream = ByteStream::new(data);

        let header = Dt1Header::new(&mut byte_stream);

        let tiles_headers: Vec<TileHeader> = (0..header.num_tiles)
            .map(|_| TileHeader::new(&mut byte_stream))
            .collect();

        let tiles: Vec<Tile> = tiles_headers
            .into_iter()
            .map(|tile_header| Tile::new(&mut byte_stream, tile_header))
            .collect();

        Self { header, tiles }
    }
}

impl Dt1Header {
    pub fn new(byte_stream: &mut ByteStream) -> Self {
        let version1 = byte_stream.stream_u32();
        let version2 = byte_stream.stream_u32();

        let mut is_dt1 = true;

        if version1 != 7 || version2 != 6 {
            is_dt1 = false;
        }

        for _ in 0..260 {
            let c = byte_stream.stream_u8();
            if c != 0 {
                is_dt1 = false;
                break;
            }
        }

        let num_tiles = byte_stream.stream_u32();

        let offset = byte_stream.stream_u32();

        if offset != 276 {
            is_dt1 = false;
        }

        if !is_dt1 {
            panic!("Not a dt1 file");
        }

        Self {
            version1,
            version2,
            num_tiles,
        }
    }
}

impl<'dt1_file> Tile<'dt1_file> {
    pub fn new(byte_stream: &mut ByteStream<'dt1_file>, tile_header: TileHeader) -> Self {
        byte_stream.set_offset(tile_header.tiles_ptr as usize);

        let sub_tiles_headers: Vec<SubTileHeader> = (0..tile_header.num_tiles)
            .map(|_| SubTileHeader::new(byte_stream))
            .collect();

        let sub_tiles: Vec<SubTile> = sub_tiles_headers
            .into_iter()
            .map(|sub_tile_header| {
                SubTile::new(byte_stream, tile_header.tiles_ptr as usize, sub_tile_header)
            })
            .collect();

        Self {
            header: tile_header,
            sub_tiles,
        }
    }

    pub fn draw<F>(&self, mut draw: F)
    where
        F: FnMut(u32, u32, u8),
    {
        let row_offset = match orientation_to_coordinate_system_type(self.header.orientation) {
            CoordinateSystemType::Positive => 0,
            CoordinateSystemType::Negative => self.header.height as i16,
        };

        for sub_tile in &self.sub_tiles {
            let sub_tile_draw = |row: u32, col: u32, val: u8| {
                let row = (sub_tile.header.y_position + row_offset) as u32 + row;
                let col = u32::from(sub_tile.header.x_position) + col;

                draw(row, col, val);
            };

            sub_tile.draw(sub_tile_draw);
        }
    }
}

fn orientation_to_coordinate_system_type(orientation: u32) -> CoordinateSystemType {
    match orientation {
        0 | 15 => CoordinateSystemType::Positive,
        _ => CoordinateSystemType::Negative,
    }
}

enum CoordinateSystemType {
    Positive,
    Negative,
}

impl TileHeader {
    pub fn new(byte_stream: &mut ByteStream) -> Self {
        let direction = byte_stream.stream_u32();

        let flags = byte_stream.stream_u32();

        let height = byte_stream.stream_i32().unsigned_abs();
        let width = byte_stream.stream_u32();

        let mut unknown_bytes_1 = [0; 4];

        for b in &mut unknown_bytes_1 {
            *b = byte_stream.stream_u8();
        }

        let orientation = byte_stream.stream_u32();
        let main_index = byte_stream.stream_u32();
        let sub_index = byte_stream.stream_u32();
        let frame_index = byte_stream.stream_u32();

        let mut unknown_bytes_2 = [0; 4];
        for b in &mut unknown_bytes_2 {
            *b = byte_stream.stream_u8();
        }

        let mut sub_tiles_flags = [0; 25];
        for b in &mut sub_tiles_flags {
            *b = byte_stream.stream_u8();
        }

        let mut unknown_bytes_3 = [0; 7];
        for b in &mut unknown_bytes_3 {
            *b = byte_stream.stream_u8();
        }

        let tiles_ptr = byte_stream.stream_u32();

        let tiles_length = byte_stream.stream_u32();

        let num_tiles = byte_stream.stream_u32();

        let mut unknown_bytes_4 = [0; 12];
        for b in &mut unknown_bytes_4 {
            *b = byte_stream.stream_u8();
        }

        Self {
            direction,
            flags,
            height,
            width,
            unknown_bytes_1,
            orientation,
            main_index,
            sub_index,
            frame_index,
            unknown_bytes_2,
            sub_tiles_flags,
            unknown_bytes_3,
            tiles_ptr,
            tiles_length,
            num_tiles,
            unknown_bytes_4,
        }
    }
}

impl<'dt1_file> SubTile<'dt1_file> {
    pub fn new(
        byte_stream: &mut ByteStream<'dt1_file>,
        tile_offset: usize,
        sub_tile_header: SubTileHeader,
    ) -> Self {
        byte_stream.set_offset(tile_offset + sub_tile_header.data_offset as usize);

        let data = byte_stream.stream_bytes(sub_tile_header.data_length);

        Self {
            header: sub_tile_header,
            data,
        }
    }

    fn draw<F>(&self, draw: F)
    where
        F: FnMut(u32, u32, u8),
    {
        if self.header.format == 1 {
            self.draw_sub_tile_isometric(draw);
        } else {
            self.draw_sub_tile_normal(draw);
        }
    }

    fn draw_sub_tile_normal<F>(&self, mut draw: F)
    where
        F: FnMut(u32, u32, u8),
    {
        let mut c = 0;
        let mut row: u32 = 0;
        let mut col_offset: u32 = 0;

        while c < self.data.len() {
            let col_jump = self.data[c];
            c += 1;

            col_offset += u32::from(col_jump);

            let num_pixels = self.data[c];
            c += 1;

            if col_jump != 0 || num_pixels != 0 {
                for col in col_offset..col_offset + u32::from(num_pixels) {
                    draw(row, col, self.data[c]);
                    c += 1;
                }

                col_offset += u32::from(num_pixels);
            } else {
                row += 1;
                col_offset = 0;
            }
        }
    }

    fn draw_sub_tile_isometric<F>(&self, mut draw: F)
    where
        F: FnMut(u32, u32, u8),
    {
        const ROW_DATA: [(u8, u8); 15] = [
            (14, 4),
            (12, 8),
            (10, 12),
            (8, 16),
            (6, 20),
            (4, 24),
            (2, 28),
            (0, 32),
            (2, 28),
            (4, 24),
            (6, 20),
            (8, 16),
            (10, 12),
            (12, 8),
            (14, 4),
        ];

        let mut c = 0;

        for (row, (col_offset, num_pixels)) in ROW_DATA.iter().enumerate() {
            for col in *col_offset..col_offset + num_pixels {
                draw(row as u32, u32::from(col), self.data[c]);
                c += 1;
            }
        }
    }
}

impl SubTileHeader {
    pub fn new(byte_stream: &mut ByteStream) -> Self {
        let x_position = byte_stream.stream_u16();
        let y_position = byte_stream.stream_i16();

        let mut unknown_bytes_1 = [0; 2];
        for b in &mut unknown_bytes_1 {
            *b = byte_stream.stream_u8();
        }

        let x_grid_position = byte_stream.stream_u8();
        let y_grid_position = byte_stream.stream_u8();

        let format = byte_stream.stream_u16();

        let data_length = byte_stream.stream_u32();

        let mut unknown_bytes_2 = [0; 2];
        for b in &mut unknown_bytes_2 {
            *b = byte_stream.stream_u8();
        }

        let data_offset = byte_stream.stream_u32();

        Self {
            x_position,
            y_position,
            unknown_bytes_1,
            x_grid_position,
            y_grid_position,
            format,
            data_length,
            unknown_bytes_2,
            data_offset,
        }
    }
}
