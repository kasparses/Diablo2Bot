use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

use crate::{
    matrix::Matrix,
    point_u16::PointU16,
    point_u8::PointU8,
    structs::{PointIndex, PointValue},
};

pub struct PatternMatcher {
    pub matrix_dimensions: PointU16,
    tree: Node,
}

enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

struct InternalNode {
    pub point: PointU8,
    pub zero_child: Option<Box<Node>>,
    pub children: HashMap<u8, Node, BuildNoHashHasher<u8>>,
}

struct LeafNode {
    pub sprite_id: u32,
    pub result: Vec<PointValue>,
}

impl PatternMatcher {
    pub fn find_map_sprites(
        &self,
        img: &Matrix,
        img_mask: &mut Matrix,
        points: &[PointU16],
    ) -> Vec<PointIndex> {
        let mut matches = Vec::new();

        points.iter().rev().for_each(|point| {
            Self::look_up(img, img_mask, &mut matches, *point, &self.tree);
        });

        matches
    }

    fn look_up(
        img: &Matrix,
        img_mask: &mut Matrix,
        matches: &mut Vec<PointIndex>,
        point: PointU16,
        node: &Node,
    ) {
        match node {
            Node::Internal(ref node) => {
                if let Some(next_node) = &node.zero_child {
                    Self::look_up(img, img_mask, matches, point, next_node);
                }

                // Hvis der er overlap fordi vi er out-of-bounds, så skal ALLE de andre pixels være korrekte.

                let value = img.get_value(point + node.point);
                if value != 0 {
                    if let Some(next_node) = node.children.get(&value) {
                        Self::look_up(img, img_mask, matches, point, next_node);
                    }

                    if img_mask.get_value(point + node.point) == 1 {
                        node.children.iter().for_each(|(node_value, next_node)| {
                            if *node_value != value {
                                Self::look_up(img, img_mask, matches, point, next_node);
                            }
                        });
                    }
                }
            }
            Node::Leaf(ref node) => {
                if Self::is_window_match(img, img_mask, point, &node.result) {
                    matches.push(PointIndex {
                        sprite_id: node.sprite_id,
                        point,
                    });

                    // Draw all the points in the img_mask
                    for point_value in &node.result {
                        img_mask.set_value(point + point_value.point, 1);
                    }
                }
            }
        }
    }

    fn is_window_match(
        img: &Matrix,
        img_mask: &Matrix,
        img_point: PointU16,
        point_values: &[PointValue],
    ) -> bool {
        let mut overlap_count = 0;
        let mut match_count = 0;
        for point_value in point_values {
            let value = img.get_value(img_point + point_value.point);

            if value == point_value.value {
                match_count += 1;
            } else if img_mask.get_value(img_point + point_value.point) == 1 {
                overlap_count += 1;
            } else {
                return false;
            }
        }

        let total_count = overlap_count + match_count;
        let ratio = match_count as f32 / total_count as f32;
        ratio >= 0.2
    }
}

mod creator {
    use super::{InternalNode, LeafNode, Node, PatternMatcher};
    use crate::{fast_hash_set::FastHashSet, point_u8::PointU8, structs::MatrixAndPoints2};
    use std::collections::HashMap;

    impl PatternMatcher {
        pub fn new(sprites: Vec<MatrixAndPoints2>) -> Self {
            Self::validate_not_equal_arrays(&sprites);
            Self::validate_arrys_equal_length(&sprites);

            let remaining_points = Self::get_initial_points(&sprites);

            let matrix_dimensions = sprites[0].matrix.dims;

            let root = Self::create_tree(sprites, &remaining_points);

            Self {
                matrix_dimensions,
                tree: root,
            }
        }

        fn validate_arrys_equal_length(patterns: &[MatrixAndPoints2]) {
            if patterns.is_empty() {
                return;
            }

            let len = patterns[0].matrix.len();

            for pattern in patterns {
                if pattern.matrix.len() != len {
                    panic!("All arrays must have the same length");
                }
            }
        }

        fn validate_not_equal_arrays(patterns: &[MatrixAndPoints2]) {
            let mut set = FastHashSet::new();

            for pattern in patterns {
                if !set.insert(&pattern.matrix) {
                    panic!("All arrays must be unique");
                }
            }
        }

        fn get_initial_points(sprites: &[MatrixAndPoints2]) -> Vec<PointU8> {
            let mut point_mask = vec![false; sprites[0].matrix.len()];

            sprites.iter().for_each(|sprite| {
                sprite.point_values.iter().for_each(|point_value| {
                    let idx = (point_value.point.row * sprite.matrix.dims.col
                        + point_value.point.col) as usize;
                    point_mask[idx] = true;
                });
            });

            let mut points: Vec<PointU8> = Vec::new();

            sprites[0].matrix.iter_points().for_each(|point| {
                let idx = (point.row * sprites[0].matrix.dims.col + point.col) as usize;
                if point_mask[idx] {
                    points.push(point.into());
                }
            });

            points
        }

        fn create_tree(sprites: Vec<MatrixAndPoints2>, remaining_points: &[PointU8]) -> Node {
            if sprites.len() == 1 {
                return Node::Leaf(LeafNode {
                    sprite_id: sprites[0].sprite_id,
                    result: sprites[0].point_values.clone(),
                });
            }

            let (point, remaining_points) =
                Self::get_best_point_and_remaining_points(&sprites, remaining_points);

            let remaining_points = remaining_points
                .iter()
                .filter(|&p| p != &point)
                .cloned()
                .collect::<Vec<PointU8>>();

            let splits = Self::split_sprites(sprites, point);

            let mut node = InternalNode {
                point,
                children: HashMap::default(),
                zero_child: None,
            };

            for (value, sprites) in splits.into_iter() {
                if value == 0 {
                    node.zero_child = Some(Box::new(Self::create_tree(sprites, &remaining_points)));
                } else {
                    node.children
                        .insert(value, Self::create_tree(sprites, &remaining_points));
                }
            }

            Node::Internal(node)
        }

        pub fn get_best_point_and_remaining_points(
            sprites: &[MatrixAndPoints2],
            remaining_points: &[PointU8],
        ) -> (PointU8, Vec<PointU8>) {
            let mut point_counts = vec![0; remaining_points.len()];

            for sprite in sprites {
                for (i, &point) in remaining_points.iter().enumerate() {
                    let value = sprite.matrix.get_value(point.into());
                    if value != 0 {
                        point_counts[i] += 1;
                    }
                }
            }

            let mut _remaining_points = Vec::new();

            let mut max_index = 0;
            let mut max_value = 0;
            for (i, &value) in point_counts.iter().enumerate() {
                if value > 0 {
                    _remaining_points.push(remaining_points[i]);
                }
                if value > max_value {
                    max_value = value;
                    max_index = i;
                }
            }

            (remaining_points[max_index], _remaining_points)
        }

        fn split_sprites(
            sprites: Vec<MatrixAndPoints2>,
            point: PointU8,
        ) -> HashMap<u8, Vec<MatrixAndPoints2>> {
            let mut children = HashMap::new();

            for sprite in sprites {
                let value = sprite.matrix.get_value(point.into());
                children.entry(value).or_insert_with(Vec::new).push(sprite);
            }

            children
        }
    }
}
