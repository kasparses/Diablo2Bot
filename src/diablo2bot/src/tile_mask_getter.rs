use std::collections::HashMap;

use crate::constants::{
    map_tile_mask_ranges::MAP_TILE_MASK_RANGES,
    map_tile_masks::{MapSpriteCell, EMPTY},
};

pub struct TileMaskGetter {
    id_to_mask: HashMap<u32, [[MapSpriteCell; 2]; 8]>,
}

impl TileMaskGetter {
    pub fn new() -> Self {
        let mut id_to_mask = HashMap::new();

        for (start, end, mask) in MAP_TILE_MASK_RANGES {
            for sprite_id in start..end {
                id_to_mask.insert(sprite_id, mask);
            }
        }

        // Validation
        let mut last_id = 0;
        for (start, end, _) in MAP_TILE_MASK_RANGES {
            assert_eq!(start, last_id);
            last_id = end;
        }

        Self { id_to_mask }
    }

    pub fn get_tile_mask(&self, sprite_id: u32) -> [[MapSpriteCell; 2]; 8] {
        match self.id_to_mask.get(&sprite_id) {
            Some(mask) => *mask,
            None => EMPTY,
        }
    }
}
