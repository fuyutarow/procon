use itertools::{iproduct, Itertools};
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input, source};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
const TAU: f64 = PI * 2.;

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ac_library_rs::{floor_sum, Dsu, MfGraph};
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, FenwickTree, MultiSet},
};
use utils::debug;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut S: [Chars; N],
    }

    let mut graph = MfGraph::new(N * M + 2);
    let source = N * M;
    let target = N * M + 1;

    let encode = |r: usize, c: usize| r * M + c;
    let decode = |idx: usize| (idx / M, idx % M);

    for (r, c) in iproduct!(0..(N), 0..(M)) {
        if S[r][c] == '#' {
            continue;
        }

        let idx = encode(r, c);

        if idx % 2 == 0 {
            graph.add_edge(source, idx, 1);

            let r = r as i32;
            let c = c as i32;
            for (r2, c2) in &[(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if 0 <= *r2 && *r2 < N as i32 && 0 <= *c2 && *c2 < M as i32 {
                    if S[*r2 as usize][*c2 as usize] == '#' {
                        continue;
                    }
                    let idx2 = encode(*r2 as usize, *c2 as usize);
                    graph.add_edge(idx, idx2, 1);
                }
            }
        } else {
            graph.add_edge(idx, target, 1);
        }
    }

    let res = graph.flow(source, target);

    for edge in graph.edges() {
        if edge.from == source || edge.to == target {
            continue;
        }

        if edge.flow == 1 {
            let from = decode(edge.from);
            let to = decode(edge.to);

            match edge.to as i32 - edge.from as i32 {
                -1 => {
                    S[from.0][from.1] = '<';
                    S[to.0][to.1] = '>';
                }
                1 => {
                    S[from.0][from.1] = '>';
                    S[to.0][to.1] = '<';
                }
                M => {
                    S[from.0][from.1] = 'V';
                    S[to.0][to.1] = '^';
                }
                _ => {
                    S[from.0][from.1] = '^';
                    S[to.0][to.1] = 'V';
                }
            }

            debug!(&from, &to);
        }
    }

    println!("{}", res);
    for s in S {
        println!("{}", s.iter().collect::<String>());
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
