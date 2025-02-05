use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use ahash::AHasher;
use nohash_hasher::BuildNoHashHasher;

pub struct FastHashSet<T> {
    hash_set: HashSet<u64, BuildNoHashHasher<u64>>,
    _marker: PhantomData<T>,
}

impl<T> FastHashSet<T>
where
    T: Hash,
{
    pub fn new() -> Self {
        Self {
            hash_set: HashSet::default(),
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, value: &T) -> bool {
        self.hash_set.insert(Self::hash(value))
    }

    pub fn contains(&mut self, value: &T) -> bool {
        self.hash_set.contains(&Self::hash(value))
    }

    fn hash(value: &T) -> u64 {
        let mut hasher = AHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
