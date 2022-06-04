// use itertools::Itertools;
// use num::integer::gcd;
// use petgraph::algo::kosaraju_scc as scc;
// use petgraph::graph::NodeIndex;
// use petgraph::{Directed, Graph};
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
// use std::cmp::{max, min, Reverse};
// use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::collections::HashMap;
// use std::f64::consts::PI;
// use std::i32::MAX as inf;
// use std::mem::swap;
// const TAU: f64 = PI * 2.;
use utils::{collections::MultiSet, debug};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        AB: [(usize, usize); N],
        CD: [(Usize1, usize); Q],
    }

    let mut members_rate_and_belong = vec![(0, 0); N];
    let mut park_book = HashMap::new();

    for (i, (rate, belong)) in AB.iter().enumerate() {
        members_rate_and_belong[i] = (*rate, *belong);
        park_book
            .entry(belong)
            .or_insert(MultiSet::new())
            .insert(*rate);
        debug!(rate, belong);
    }
    debug!(park_book);

    for (c, d) in &CD {
        let (rate, belong) = &members_rate_and_belong[*c];

        park_book
            .entry(belong)
            .or_insert(MultiSet::new())
            .remove(rate);
        park_book.entry(d).or_insert(MultiSet::new()).insert(*rate);

        debug!(park_book);

        let res = park_book.iter().flat_map(|(k, v)| v.max()).min().unwrap();

        println!("{}", res);
    }
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
