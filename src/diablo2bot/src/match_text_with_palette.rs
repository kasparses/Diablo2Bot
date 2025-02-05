use std::collections::HashMap;

use crate::{
    enums::{palette::Palette, quality::Quality},
    font_char_map::get_non_control_ascii_char_font_map,
    font_matcher::FontMatcher,
    game::Game,
    image::Image,
    matrix::Matrix,
    pal_pl2::PalPl2Bytes,
    quality_palette::QualityPalette,
    structs::Item,
};

pub fn get_font_char_map(g: &mut Game) -> HashMap<char, Matrix> {
    let font_dc6_bytes = g.archives.extract_font_16_bytes().unwrap();
    let font_dc6_file = font_dc6_bytes.parse();
    get_non_control_ascii_char_font_map(&font_dc6_file)
}

pub fn match_unique_text_with_palette(
    g: &mut Game,
    font_char_map: &HashMap<char, Matrix>,
    img: &Image,
    palette: Palette,
) -> Vec<Item> {
    let pal_pl2_bytes = g.archives.extract_pal_pl2_bytes(palette).unwrap();

    let act_palette_bytes = pal_pl2_bytes.extract_act_palette_bytes();
    let pixel_palette = act_palette_bytes.get_pixel_palette();
    let palette_transformer = act_palette_bytes.get_palette_transformer(&pixel_palette);

    let font_symbol_matcher = get_unique_color_font_symbol_matcher(font_char_map, &pal_pl2_bytes);

    let matrix = img.to_matrix(&palette_transformer);

    font_symbol_matcher.match_image_items(&matrix)
}

pub fn get_unique_color_font_symbol_matcher(
    font_char_map: &HashMap<char, Matrix>,
    pal_pl2_bytes: &PalPl2Bytes,
) -> FontMatcher {
    let quality_palettes: Vec<QualityPalette> = pal_pl2_bytes
        .extract_font_quality_palette_bytes()
        .get_palettes()
        .into_iter()
        .filter(|qp| qp.color == Quality::Unique)
        .collect();

    FontMatcher::new(&quality_palettes, font_char_map)
}
