use crate::bit_stream::BitStream;

#[derive(Debug)]
pub struct BitStreamSizes {
    equal_cell: Option<u32>,
    pixel_mask: u32,
    encoding_type_raw_pixel: Option<EncodingTypeRawPixelBitStreamSizes>,
}

#[derive(Debug)]
struct EncodingTypeRawPixelBitStreamSizes {
    encoding_type: u32,
    raw_pixel: u32,
}

#[derive(Debug)]
pub struct BitStreams<'dcc_file> {
    pub equal_cell: Option<BitStream<'dcc_file>>,
    pub pixel_mask: BitStream<'dcc_file>,
    pub encoding_type_raw_pixel: Option<EncodingTypeRawPixelBitStreams<'dcc_file>>,
    pub pixel_code_and_displacment: BitStream<'dcc_file>,
}

#[derive(Debug)]
pub struct EncodingTypeRawPixelBitStreams<'dcc_file> {
    pub encoding_type: BitStream<'dcc_file>,
    pub raw_pixel: BitStream<'dcc_file>,
}

impl BitStreamSizes {
    pub fn load(bs: &mut BitStream, compression_flag: u32) -> Self {
        let equal_cell = if compression_flag & 2 != 0 {
            Some(bs.stream_bits(20))
        } else {
            None
        };

        let pixel_mask = bs.stream_bits(20);

        let encoding_type_raw_pixel = if compression_flag & 1 != 0 {
            let encoding_type = bs.stream_bits(20);
            let raw_pixel = bs.stream_bits(20);
            Some(EncodingTypeRawPixelBitStreamSizes {
                encoding_type,
                raw_pixel,
            })
        } else {
            None
        };

        Self {
            equal_cell,
            pixel_mask,
            encoding_type_raw_pixel,
        }
    }
}

impl<'bytes> BitStreams<'bytes> {
    pub fn load(bs: &mut BitStream, bs_sizes: &BitStreamSizes, data: &'bytes [u8]) -> Self {
        let mut bit_offset = (bs.byte_offset * 8) as u32 + u32::from(bs.bit_offset);

        let mut create_bitstream = |size: u32| -> BitStream<'bytes> {
            let bitstream = BitStream::new_with_bit_offset(
                &data[(bit_offset / 8) as usize..],
                (bit_offset % 8) as u8,
            );
            bit_offset += size;
            bitstream
        };

        let equal_cell = bs_sizes.equal_cell.map(&mut create_bitstream);

        let pixel_mask = create_bitstream(bs_sizes.pixel_mask);

        let encoding_type_raw_pixel = match &bs_sizes.encoding_type_raw_pixel {
            Some(sizes) => {
                let encoding_type = create_bitstream(sizes.encoding_type);
                let raw_pixel = create_bitstream(sizes.raw_pixel);
                Some(EncodingTypeRawPixelBitStreams {
                    encoding_type,
                    raw_pixel,
                })
            }
            None => None,
        };

        let pixel_code_and_displacment = create_bitstream(0);

        Self {
            equal_cell,
            pixel_mask,
            encoding_type_raw_pixel,
            pixel_code_and_displacment,
        }
    }
}
