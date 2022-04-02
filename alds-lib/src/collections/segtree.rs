//! https://github.com/ngtkana/ac-adapter-rs/blob/master/crates/segtree/src/lib.rs
//! セグメント木です。
//!
//! # 愉快な仲間たち
//!
//! - [`Segtree`] : 本体
//! - [`Ops`] : 演算
//!
//!
//! # Examples
//!
//! ```
//! use segtree::{Segtree, Ops};
//!
//! // 演算の実装
//! enum O {}
//! impl Ops for O {
//!     type Value = i32;
//!     fn op(x: &i32, y: &i32) -> i32 {
//!         x + y
//!     }
//!     fn identity() -> i32 {
//!         0
//!     }
//! }
//!
//! // 本体の使い方
//! let mut seg = Segtree::<O>::new(vec![0; 5]);
//! assert_eq!(&[0, 0, 0, 0, 0], seg.as_ref());
//! *seg.entry(1) = 2;
//! *seg.entry(3) = 5;
//! assert_eq!(&[0, 2, 0, 5, 0], seg.as_ref());
//! assert_eq!(seg.fold(..), 7);
//! ```
//!
use std::{
    collections::VecDeque,
    fmt::Debug,
    iter::{repeat_with, Cloned, FromIterator},
    ops::{Bound, Deref, DerefMut, Index, Range, RangeBounds},
    slice::SliceIndex,
};

use super::super::monoid::Monoid;

pub struct Segtree<M: Monoid> {
    table: Box<[M::Value]>,
}

impl<M: Monoid> Segtree<M> {
    pub fn new() -> Self {
        Self {
            table: Box::new([]),
        }
    }

    /// 表している配列が空であるときに `true` です。
    ///
    /// ```
    /// # use monoid::Monoid;
    /// # enum Sum {}
    /// # impl Monoid for Sum {
    /// #     type Value = i32;
    /// #     fn id() -> i32 {
    /// #         0
    /// #     }
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let seg = Segtree::<Sum>::from(vec![0, 1, 2]);
    /// assert_eq!(seg.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// 表している配列を返します。
    ///
    /// ```
    /// # use segtree::Ops;
    /// # enum O {}
    /// # impl Ops for O {
    /// #     type Value = i32;
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// #     fn identity() -> i32 {
    /// #         0
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let seg = Segtree::<O>::new(vec![0, 1, 2]);
    /// assert_eq!(seg.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.table.len() / 2
    }

    pub fn insert(&mut self, idx: usize, value: M::Value) {
        *self.entry(idx) = value;
    }

    /// 与えられた範囲で畳み込みます。
    ///
    /// # Panics
    ///
    /// 標準ライブラリの [`SliceIndex`] と同じ条件でパニックします。
    ///
    /// # Output
    ///
    /// `range = [l, r[` のとき、`op(a[l], ..., a[r - 1])` を返します。ただし空のときには
    /// `identity()` を返します。
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use segtree::Ops;
    /// # enum O {}
    /// # impl Ops for O {
    /// #     type Value = i32;
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// #     fn identity() -> i32 {
    /// #         0
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let seg = Segtree::<O>::new(vec![0, 1, 2]);
    /// assert_eq!(seg.fold(1..3), 3);
    /// ```
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {
        let mut left = M::id();
        let mut right = M::id();
        let Range { mut start, mut end } = into_slice_range(self.len(), range);
        if end < start {
            segtree_index_order_fail(start, end);
        }
        if self.len() < end {
            segtree_end_index_len_fail(end, self.len());
        }
        start += self.len();
        end += self.len();
        while start != end {
            if start % 2 == 1 {
                left = M::op(&left, &self.table[start]);
                start += 1;
            }
            if end % 2 == 1 {
                end -= 1;
                right = M::op(&self.table[end], &right);
            }
            start /= 2;
            end /= 2;
        }
        M::op(&left, &right)
    }
    /// 命題の成り立つ最大の区間を探します。
    ///
    /// # Panics
    ///
    /// 標準ライブラリの [`SliceIndex`] と同じ条件でパニックします。
    ///
    ///
    /// # Output
    ///
    /// 表している配列を `a` とするとき、次を満たす `i` が存在するのでひとつ返します。
    ///
    /// - `i == l || pred(self.fold(l..i))`
    /// - `i == r || !pred(self.fold(l..=i))`
    ///
    /// とくに `pred(self.fold(l..i))` が `i` に関して `true` から `false`
    /// に向かって単調ならば、`pred(self.fold(l..i))` を満たす最大の `i` です。
    ///
    ///
    /// 0   1   2   3   4   5   6
    /// | 0 | 1 | 2 | 5 | 3 | 2 |
    /// |   | T | T | F | T |   |
    ///             ^   ^
    ///      max_right  min_left
    ///
    /// ```
    /// let mut segtree = Segtree::<Max<usize>>::from(vec![0, 1, 2, 5, 3, 2]);    ///
    /// max_right(1..5, |&x| x < 4) == 3
    /// min_left(1..5, |&x| x < 4) == 4
    /// ```
    pub fn max_right(
        &self,
        range: impl RangeBounds<usize>,
        mut pred: impl FnMut(&M::Value) -> bool,
    ) -> usize {
        let Range { mut start, mut end } = into_slice_range(self.len(), range);
        if start == end {
            start
        } else {
            start += self.len();
            end += self.len();
            let orig_end = end;
            let mut crr = M::id();
            let mut shift = 0;
            while start != end {
                if start % 2 == 1 {
                    let nxt = M::op(&crr, &self.table[start]);
                    if !pred(&nxt) {
                        return self.max_right_subtree(crr, start, pred);
                    }
                    crr = nxt;
                    start += 1;
                }
                start /= 2;
                end /= 2;
                shift += 1;
            }
            for p in (0..shift).rev() {
                let end = (orig_end >> p) - 1;
                if end % 2 == 0 {
                    let nxt = M::op(&crr, &self.table[end]);
                    if !pred(&nxt) {
                        return self.max_right_subtree(crr, end, pred);
                    }
                    crr = nxt;
                }
            }
            orig_end - self.len()
        }
    }
    fn max_right_subtree(
        &self,
        mut crr: M::Value,
        mut root: usize,
        mut pred: impl FnMut(&M::Value) -> bool,
    ) -> usize {
        while root < self.len() {
            let nxt = M::op(&crr, &self.table[root * 2]);
            root = if pred(&nxt) {
                crr = nxt;
                root * 2 + 1
            } else {
                root * 2
            };
        }
        root - self.len()
    }
    /// 命題の成り立つ最大の区間を探します。
    ///
    /// # Panics
    ///
    /// 標準ライブラリの [`SliceIndex`] と同じ条件でパニックします。
    ///
    ///
    /// # Output
    ///
    /// 表している配列を `a` とするとき、次を満たす `i` が存在するのでひとつ返します。
    ///
    /// - `i == r || pred(self.fold(l..i))`
    /// - `i == l || !pred(self.fold(l-1..i))`
    ///
    /// とくに `pred(self.fold(i..r))` が `i` に関して `false` から `true`
    /// に向かって単調ならば、`pred(self.fold(i..r))` を満たす最小の `i` です。
    ///
    pub fn min_left(
        &self,
        range: impl RangeBounds<usize>,
        mut pred: impl FnMut(&M::Value) -> bool,
    ) -> usize {
        let Range { mut start, mut end } = into_slice_range(self.len(), range);
        if start == end {
            start
        } else {
            start += self.len();
            end += self.len();
            let orig_start_m1 = start - 1;
            let mut crr = M::id();
            let mut shift = 0;
            while start != end {
                if end % 2 == 1 {
                    end -= 1;
                    let nxt = M::op(&self.table[end], &crr);
                    if !pred(&nxt) {
                        return self.min_left_subtree(crr, end, pred);
                    }
                    crr = nxt;
                }
                start = (start + 1) >> 1;
                end >>= 1;
                shift += 1;
            }
            for p in (0..shift).rev() {
                let start = (orig_start_m1 >> p) + 1;
                if start % 2 == 1 {
                    let nxt = M::op(&self.table[start], &crr);
                    if !pred(&nxt) {
                        return self.min_left_subtree(crr, start, pred);
                    }
                    crr = nxt;
                }
            }
            orig_start_m1 + 1 - self.len()
        }
    }
    fn min_left_subtree(
        &self,
        mut crr: M::Value,
        mut root: usize,
        mut pred: impl FnMut(&M::Value) -> bool,
    ) -> usize {
        while root < self.len() {
            let nxt = M::op(&self.table[root * 2 + 1], &crr);
            root = if pred(&nxt) {
                crr = nxt;
                root * 2
            } else {
                root * 2 + 1
            };
        }
        root + 1 - self.len()
    }
    /// 要素の可変ハンドラを返します。
    ///
    /// # Panics
    ///
    /// [`Deref`] か [`Drop`] するまではパニックしません。
    /// [`Deref`] か [`Drop`] でパニックする条件は [`SliceIndex`] と同じです。
    ///
    /// # Output
    ///
    /// `range = [l, r[` のとき、`op(a[l], ..., a[r - 1])` を返します。ただし空のときには
    /// `identity()` を返します。
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use segtree::Ops;
    /// # enum O {}
    /// # impl Ops for O {
    /// #     type Value = i32;
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// #     fn identity() -> i32 {
    /// #         0
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let mut seg = Segtree::<O>::new(vec![0, 1, 2]);
    /// *seg.entry(0) = 10;
    /// assert_eq!(seg.fold(..), 13);
    /// ```
    pub fn entry(&mut self, idx: usize) -> Entry<'_, M> {
        Entry { idx, seg: self }
    }
    /// 表している配列をスライスで返します。
    ///
    /// # Examples
    ///
    /// ```
    /// # use segtree::Ops;
    /// # enum O {}
    /// # impl Ops for O {
    /// #     type Value = i32;
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// #     fn identity() -> i32 {
    /// #         0
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let seg = Segtree::<O>::new(vec![0, 1, 2]);
    /// assert_eq!(seg.as_slice(), &[0, 1, 2]);
    /// ```
    pub fn as_slice(&self) -> &[M::Value] {
        self.as_ref()
    }
    /// 表している配列を可変スライスで返します。
    ///
    /// # Examples
    ///
    /// ```
    /// # use segtree::Ops;
    /// # enum O {}
    /// # impl Ops for O {
    /// #     type Value = i32;
    /// #     fn op(x: &i32, y: &i32) -> i32 {
    /// #         x + y
    /// #     }
    /// #     fn identity() -> i32 {
    /// #         0
    /// #     }
    /// # }
    /// use segtree::Segtree;
    ///
    /// let mut seg = Segtree::<O>::new(vec![0, 1, 2]);
    /// assert_eq!(seg.as_slice_mut(), &mut [0, 1, 2]);
    /// ```
    pub fn as_slice_mut(&mut self) -> &mut [M::Value] {
        self.as_mut()
    }
}

impl<M: Monoid> Default for Segtree<M> {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////////
// 要素アクセス
////////////////////////////////////////////////////////////////////////////////
impl<M: Monoid, Idx: SliceIndex<[M::Value], Output = M::Value>> Index<Idx> for Segtree<M> {
    type Output = M::Value;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_slice()[index]
    }
}
/// [`Segtree`] のエントリー型です。
///
/// [`impl Deref`](struct.Entry.html#impl-Deref) で要素にアクセスして、[`impl
/// Drop`](struct.Entry.html#impl-Drop) で再計算をします。
pub struct Entry<'a, M: Monoid> {
    idx: usize,
    seg: &'a mut Segtree<M>,
}
impl<'a, M: Monoid> Drop for Entry<'a, M> {
    fn drop(&mut self) {
        self.idx += self.seg.len();
        self.idx /= 2;
        while self.idx != 0 {
            self.seg.table[self.idx] = M::op(
                &self.seg.table[2 * self.idx],
                &self.seg.table[2 * self.idx + 1],
            );
            self.idx /= 2;
        }
    }
}
impl<M: Monoid> Deref for Entry<'_, M> {
    type Target = M::Value;
    fn deref(&self) -> &Self::Target {
        &self.seg.as_slice()[self.idx]
    }
}
impl<M: Monoid> DerefMut for Entry<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seg.as_slice_mut()[self.idx]
    }
}

////////////////////////////////////////////////////////////////////////////////
// 変換
////////////////////////////////////////////////////////////////////////////////
impl<M: Monoid> From<Vec<M::Value>> for Segtree<M> {
    fn from(v: Vec<M::Value>) -> Self {
        let iter = v.into_iter();
        let n = iter.len();
        let mut table = repeat_with(M::Value::default).take(n).collect::<Vec<_>>();
        table.extend(iter);
        (1..n)
            .rev()
            .for_each(|i| table[i] = M::op(&table[2 * i], &table[2 * i + 1]));
        let table = table.into_boxed_slice();
        Self { table }
    }
}

impl<M: Monoid> FromIterator<M::Value> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let mut v = iter.into_iter().collect::<VecDeque<_>>();
        let n = v.len();
        let mut table = repeat_with(M::Value::default)
            .take(n)
            .collect::<VecDeque<_>>();
        table.append(&mut v);
        (1..n)
            .rev()
            .for_each(|i| table[i] = M::op(&table[2 * i], &table[2 * i + 1]));
        let table = Vec::from(table).into_boxed_slice();
        Self { table }
    }
}

impl<M: Monoid> AsRef<[M::Value]> for Segtree<M> {
    fn as_ref(&self) -> &[M::Value] {
        &self.table[self.len()..]
    }
}

impl<M: Monoid> AsMut<[M::Value]> for Segtree<M> {
    fn as_mut(&mut self) -> &mut [M::Value] {
        let n = self.len();
        &mut self.table[n..]
    }
}

////////////////////////////////////////////////////////////////////////////////
// フォーマット
////////////////////////////////////////////////////////////////////////////////
impl<M: Monoid> Debug for Segtree<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

////////////////////////////////////////////////////////////////////////////////
// プライベート - RangeBounds 関連
////////////////////////////////////////////////////////////////////////////////
fn into_slice_range(len: usize, range: impl RangeBounds<usize>) -> Range<usize> {
    #[allow(clippy::redundant_closure)]
    let start = match range.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => start
            .checked_add(1)
            .unwrap_or_else(|| slice_start_index_overflow_fail()),
        Bound::Unbounded => 0,
    };
    #[allow(clippy::redundant_closure)]
    let end = match range.end_bound() {
        Bound::Included(&end) => end
            .checked_add(1)
            .unwrap_or_else(|| slice_end_index_overflow_fail()),
        Bound::Excluded(&end) => end,
        Bound::Unbounded => len,
    };
    start..end
}
fn segtree_end_index_len_fail(index: usize, len: usize) -> ! {
    panic!(
        "range end index {} out of range for segtree of length {}",
        index, len
    );
}
fn segtree_index_order_fail(index: usize, end: usize) -> ! {
    panic!("segtree index starts at {} but ends at {}", index, end);
}
fn slice_start_index_overflow_fail() -> ! {
    panic!("attempted to index slice from after maximum usize");
}
fn slice_end_index_overflow_fail() -> ! {
    panic!("attempted to index slice up to maximum usize");
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use {
        super::{Monoid, Segtree},
        itertools::Itertools,
        rand::{prelude::StdRng, Rng, SeedableRng},
        std::{iter::once, mem::swap},
    };

    #[test]
    fn test_fold() {
        enum Op {}
        impl Monoid for Op {
            type Value = i32;
            fn op(lhs: &i32, rhs: &i32) -> i32 {
                lhs + rhs
            }
            fn id() -> i32 {
                0
            }
        }
        let a = vec![1, 2, 4, 8, 16];
        let seg = Segtree::<Op>::from(a.clone());
        let n = a.len();
        (0..=n)
            .cartesian_product(0..=n)
            .flat_map(|(l, r)| {
                let mut v = Vec::new();
                if l <= r {
                    v.push((seg.fold(l..r), &a[l..r]));
                }
                if l <= r && r < n {
                    v.push((seg.fold(l..=r), &a[l..=r]));
                }
                if l != 0 && l < r {
                    let range = (Bound::Excluded(l), Bound::Excluded(r));
                    v.push((seg.fold(range), &a[l + 1..r]));
                }
                if l != 0 && l <= r && r < n {
                    let range = (Bound::Excluded(l), Bound::Included(r));
                    v.push((seg.fold(range), &a[l + 1..=r]));
                }
                v
            })
            .chain((0..=n).flat_map(|i| {
                let mut v = vec![(seg.fold(i..), &a[i..]), (seg.fold(..i), &a[..i])];
                if i != n {
                    v.push((seg.fold(..=i), &a[..=i]));
                }
                if i != 0 && i != n {
                    let range = (Bound::Excluded(i), Bound::Unbounded);
                    v.push((seg.fold(range), &a[i + 1..]));
                }
                v
            }))
            .chain(once((seg.fold(..), a.as_ref())))
            .for_each(|(result, expected)| {
                let expected = expected.iter().fold(Op::id(), |acc, x| Op::op(&acc, x));
                assert_eq!(result, expected);
            });
    }

    #[test]
    fn test_cat() {
        enum Op {}
        impl Monoid for Op {
            type Value = String;
            fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
                lhs.chars().chain(rhs.chars()).collect()
            }
            fn id() -> Self::Value {
                String::new()
            }
        }
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let n = rng.gen_range(1..=10);
            let mut a = (0..)
                .take(n)
                .map(|x| (x + b'a') as char)
                .take(n)
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            let mut seg = Segtree::<Op>::from(a.iter().cloned());
            for _ in 0..2 * n {
                match rng.gen_range(0..2) {
                    0 => {
                        let i = rng.gen_range(0..n);
                        let x = rng.gen_range('a'..='z').to_string();
                        a[i] = x.clone();
                        *seg.entry(i) = x;
                    }
                    1 => {
                        let mut l = rng.gen_range(0..n);
                        let mut r = rng.gen_range(0..n + 1);
                        if r <= l {
                            swap(&mut l, &mut r);
                            r += 1;
                        }
                        let expected = a[l..r].iter().fold(Op::id(), |acc, x| Op::op(&acc, x));
                        let result = seg.fold(l..r);
                        assert_eq!(expected, result);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_max_right() {
        enum Op {}
        impl Monoid for Op {
            type Value = String;
            fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
                lhs.chars().chain(rhs.chars()).collect()
            }
            fn id() -> Self::Value {
                String::new()
            }
        }
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let n = rng.gen_range(1..=10);
            let a = (0..)
                .take(n)
                .map(|x| (x + b'a') as char)
                .take(n)
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            let seg = Segtree::<Op>::from(a.iter().cloned());
            for _ in 0..2 * n {
                let mut l = rng.gen_range(0..n);
                let mut r = rng.gen_range(0..n + 1);
                if r <= l {
                    swap(&mut l, &mut r);
                    r += 1;
                }
                let expected = rng.gen_range(l..=r);
                let string = a[l..expected]
                    .iter()
                    .flat_map(|string| string.chars())
                    .collect::<String>();
                let result = seg.max_right(l..r, |t| t.len() <= string.len());
                assert_eq!(expected, result);
            }
        }
    }

    #[test]
    fn test_max_left() {
        enum Op {}
        impl Monoid for Op {
            type Value = String;
            fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
                lhs.chars().chain(rhs.chars()).collect()
            }
            fn id() -> Self::Value {
                String::new()
            }
        }
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let n = rng.gen_range(1..=10);
            let a = (0..)
                .take(n)
                .map(|x| (x + b'a') as char)
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            let seg = Segtree::<Op>::from(a.iter().cloned());
            for _ in 0..2 * n {
                let mut l = rng.gen_range(0..n);
                let mut r = rng.gen_range(0..n + 1);
                if r <= l {
                    swap(&mut l, &mut r);
                    r += 1;
                }
                let expected = rng.gen_range(l..=r);
                let string = a[expected..r]
                    .iter()
                    .flat_map(|string| string.chars())
                    .collect::<String>();
                let result = seg.min_left(l..r, |t| t.len() <= string.len());
                assert_eq!(expected, result);
            }
        }
    }
}
