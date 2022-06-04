use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

use utils::{
    collections::MultiSet,
    {chmax, chmin, debug},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i32; N],
        B: [i32; N],
        C: [i32; M],
        D: [i32; M],
    }

    let mut AB = A
        .into_iter()
        .zip(B.into_iter())
        .collect::<Vec<(i32, i32)>>();

    let mut CD = C
        .into_iter()
        .zip(D.into_iter())
        .collect::<Vec<(i32, i32)>>();

    AB.sort();
}

mod utils {
    pub mod collections {
        use std::collections::BTreeMap;

        #[derive(Debug)]
        pub struct MultiSet<T>(BTreeMap<T, usize>);

        impl<T: Ord> MultiSet<T> {
            pub fn new() -> Self {
                Self(BTreeMap::new())
            }

            pub fn insert(&mut self, v: T) {
                *self.0.entry(v).or_default() += 1;
            }

            pub fn remove(&mut self, v: &T) {
                let r = self.0.get_mut(v).unwrap();
                *r -= 1;
                if *r == 0 {
                    self.0.remove(v);
                }
            }

            pub fn min(&self) -> Option<&T> {
                self.0.iter().nth(0).map(|r| r.0)
            }

            pub fn max(&self) -> Option<&T> {
                self.0.iter().last().map(|r| r.0)
            }
        }
    }

    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

    pub(crate) use debug;

    macro_rules! chmin {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_min = _min!($($cmps),+);
            if $base > cmp_min {
                $base = cmp_min;
                true
            } else {
                false
            }
        }};
    }
    pub(crate) use chmin;

    macro_rules! chmax {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_max = _max!($($cmps),+);
            if $base < cmp_max {
                $base = cmp_max;
                true
            } else {
                false
            }
        }};
    }
    pub(crate) use chmax;

    macro_rules! _min {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::min($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::min($a, min!($($rest),+))
        }};
    }

    macro_rules! _max {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::max($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::max($a, max!($($rest),+))
        }};
    }
}
