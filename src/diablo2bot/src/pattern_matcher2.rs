use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

use crate::{
    fast_hash_set::FastHashSet,
    matrix::Matrix,
    point_u16::PointU16,
    point_u8::PointU8,
    structs::{PointValue, TrieDataTrait, TrieOutput},
};

pub struct PatternMatcher2<T>
where
    T: TrieDataTrait + Clone,
{
    matrix_dimensions: PointU16,
    tree: Node<T>,
}

enum Node<T> {
    Internal(InternalNode<T>),
    Leaf(LeafNode<T>),
}

struct InternalNode<T> {
    pub point: PointU8,
    pub children: HashMap<u8, Node<T>, BuildNoHashHasher<u8>>,
}

struct LeafNode<T> {
    pub result: T,
}

type ValuePoints =
    HashMap<u8, Vec<PointU16>, std::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<u8>>>;

impl<T> PatternMatcher2<T>
where
    T: TrieDataTrait + Clone,
{
    pub fn look_up(&self, img: &Matrix) -> Vec<TrieOutput<T::Output>> {
        let points = img.get_window_points(self.matrix_dimensions);
        self.look_up_points(img, points)
    }

    fn look_up_points(&self, img: &Matrix, points: Vec<PointU16>) -> Vec<TrieOutput<T::Output>> {
        let mut matches = Vec::new();
        Self::_look_up(img, &mut matches, points, &self.tree);
        matches
    }

    pub fn look_up_point(&self, img: &Matrix, point: PointU16) -> Vec<TrieOutput<T::Output>> {
        let points = vec![point];
        let mut matches = Vec::new();
        Self::_look_up(img, &mut matches, points, &self.tree);
        matches
    }

    fn _look_up(
        img: &Matrix,
        matches: &mut Vec<TrieOutput<T::Output>>,
        points: Vec<PointU16>,
        node: &Node<T>,
    ) {
        match node {
            Node::Internal(ref node) => {
                for (value, points) in Self::split_sprites_(img, points, node) {
                    if let Some(next_node) = node.children.get(&value) {
                        Self::_look_up(img, matches, points, next_node);
                    }
                }
            }
            Node::Leaf(ref node) => {
                matches.extend(
                    points
                        .iter()
                        .filter(|&point| {
                            Self::is_window_match(img, *point, node.result.get_point_values())
                        })
                        .map(|point| node.result.get_output(*point)),
                );
            }
        }
    }

    fn is_window_match(img: &Matrix, img_point: PointU16, window: &[PointValue]) -> bool {
        window
            .iter()
            .all(|point_value| img.get_value(img_point + point_value.point) == point_value.value)
    }

    fn get_mask(node: &InternalNode<T>) -> [bool; 256] {
        let mut mask = [false; 256];
        for value in node.children.keys() {
            if *value != 0 {
                mask[*value as usize] = true;
            }
        }
        mask
    }

    fn split_sprites_(img: &Matrix, points: Vec<PointU16>, node: &InternalNode<T>) -> ValuePoints {
        let mask = Self::get_mask(node);
        let mut children = HashMap::default();

        for point in &points {
            let value = img.get_value(*point + node.point);
            if mask[value as usize] {
                children.entry(value).or_insert_with(Vec::new).push(*point);
            }
        }

        children.insert(0, points);

        children
    }

    pub fn new(sprites: Vec<T>) -> Self {
        Self::validate_not_equal_arrays(&sprites);
        Self::validate_arrys_equal_length(&sprites);

        let remaining_points = Self::get_initial_points(&sprites);

        let matrix_dimensions = sprites[0].get_matrix().dims;

        let root = Self::create_tree(sprites, &remaining_points);

        Self {
            matrix_dimensions,
            tree: root,
        }
    }

    fn validate_arrys_equal_length(patterns: &[T]) {
        if patterns.is_empty() {
            return;
        }

        let len = patterns[0].get_matrix().len();

        for pattern in patterns {
            if pattern.get_matrix().len() != len {
                panic!("All arrays must have the same length");
            }
        }
    }

    fn validate_not_equal_arrays(patterns: &[T]) {
        let mut set = FastHashSet::new();

        for pattern in patterns {
            if !set.insert(&pattern.get_matrix()) {
                panic!("All arrays must be unique");
            }
        }
    }

    fn get_initial_points(sprites: &[T]) -> Vec<PointU8> {
        let mut point_mask = vec![false; sprites[0].get_matrix().len()];

        sprites.iter().for_each(|sprite| {
            sprite.get_point_values().iter().for_each(|point_value| {
                let idx = (point_value.point.row * sprite.get_matrix().dims.col
                    + point_value.point.col) as usize;
                point_mask[idx] = true;
            });
        });

        let mut points: Vec<PointU8> = Vec::new();

        sprites[0].get_matrix().iter_points().for_each(|point| {
            let idx = (point.row * sprites[0].get_matrix().dims.col + point.col) as usize;
            if point_mask[idx] {
                points.push(point.into());
            }
        });

        points
    }

    fn create_tree(sprites: Vec<T>, remaining_points: &[PointU8]) -> Node<T> {
        if sprites.len() == 1 {
            return Node::Leaf(LeafNode {
                result: sprites[0].to_owned(),
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
        };

        for (value, sprites) in splits.into_iter() {
            node.children
                .insert(value, Self::create_tree(sprites, &remaining_points));
        }

        Node::Internal(node)
    }

    pub fn get_best_point_and_remaining_points(
        sprites: &[T],
        remaining_points: &[PointU8],
    ) -> (PointU8, Vec<PointU8>) {
        let mut point_counts = vec![0; remaining_points.len()];

        for sprite in sprites {
            for (i, &point) in remaining_points.iter().enumerate() {
                let value = sprite.get_matrix().get_value(point.into());
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

    fn split_sprites(sprites: Vec<T>, point: PointU8) -> HashMap<u8, Vec<T>> {
        let mut children = HashMap::new();

        for sprite in sprites {
            let value = sprite.get_matrix().get_value(point.into());
            children.entry(value).or_insert_with(Vec::new).push(sprite);
        }

        children
    }
}
