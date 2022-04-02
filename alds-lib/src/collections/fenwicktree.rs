// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/fenwicktree.rs
// https://atcoder.jp/contests/practice2/submissions/16629745

use num::{zero, Integer};
use std::ops::AddAssign;

#[derive(Debug, Default, Clone)]
pub struct FenwickTree<T> {
    ary: Vec<T>,
    e: T,
}

impl<T: Copy + AddAssign + Integer> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            ary: vec![zero(); n],
            e: zero(),
        }
    }

    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }

    /// performs data[idx] += val;
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        idx += 1;
        while idx <= self.ary.len() {
            self.ary[idx - 1] += val.clone();
            idx += self.len_of(idx);
        }
    }

    pub fn len_of(&self, idx: usize) -> usize {
        idx & idx.wrapping_neg()
    }

    /// Returns data[l] + ... + data[r - 1].
    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(r) - self.accum(l)
    }

    pub fn len(&self) -> usize {
        self.ary.len()
    }
}

impl<T: Copy + AddAssign + Integer> From<Vec<T>> for FenwickTree<T> {
    fn from(v: Vec<T>) -> Self {
        let mut bit = Self::new(v.len());
        for (i, e) in v.into_iter().enumerate() {
            bit.add(i, e);
        }
        bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fenwick_tree_works() {
        let mut bit = FenwickTree::<i64>::new(5);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0, 5), 15);
        assert_eq!(bit.sum(0, 4), 10);
        assert_eq!(bit.sum(1, 3), 5);
    }
}
