// https://github.com/tanakh/competitive-rs/blob/master/src/collections.rs

use std::collections::{
    btree_map::{Iter, Keys},
    BTreeMap,
};

#[derive(Debug)]
pub struct MultiSet<T>(pub BTreeMap<T, usize>);

impl<T: Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn keys(&self) -> Keys<T, usize> {
        self.0.keys()
    }

    pub fn get(&self, v: &T) -> Option<&usize> {
        self.0.get(v)
    }

    pub fn get_mut(&mut self, v: &T) -> Option<&mut usize> {
        self.0.get_mut(v)
    }

    pub fn contains(&self, v: &T) -> bool {
        self.0.contains_key(v)
    }

    pub fn insert(&mut self, v: T) {
        self.insert_times(v, 1);
    }

    pub fn insert_times(&mut self, v: T, n: usize) {
        *self.0.entry(v).or_default() += n;
    }

    pub fn remove(&mut self, v: &T) {
        self.remove_times(v, 1);
    }

    pub fn remove_times(&mut self, v: &T, n: usize) {
        let r = self.0.get_mut(v).unwrap();
        *r = 0.min(*r - n);
        if *r == 0 {
            self.0.remove(v);
        }
    }

    pub fn remove_all(&mut self, v: &T) {
        let r = self.0.get_mut(v).unwrap();
        self.0.remove(v);
    }

    pub fn min(&self) -> Option<&T> {
        self.0.iter().next().map(|r| r.0)
    }

    pub fn max(&self) -> Option<&T> {
        self.0.iter().last().map(|r| r.0)
    }

    pub fn iter(&self) -> Iter<T, usize> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.values().sum()
    }

    pub fn len_unique(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
