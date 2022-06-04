#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use itertools::{Combinations, Itertools};
use num::integer::gcd;
use petgraph::{algo::kosaraju_scc as scc, graph::NodeIndex, Directed, Graph};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
use std::{
    cmp::{max as _max, min as _min, Reverse},
    fmt::Binary,
};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    iter::FromIterator,
};
const TAU: f64 = PI * 2.;

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ac_library_rs::{floor_sum, Dsu, MfGraph, ModInt1000000007, ModInt998244353};
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, FenwickTree, MultiSet, Segtree},
    max, min,
    monoid::{Max, Min, Monoid, Product, Sum},
};
use utils::debug;

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(usize, usize); N],
    }

    let vv = AB.into_iter().map(|(a, b)| (b, a - b)).collect_vec();
    let mut heap = BinaryHeap::from(vv);

    let mut ans = 0;
    for _ in 0..K {
        match heap.pop() {
            Some((b, 0)) => {
                ans += b;
            }
            Some((a, b)) => {
                ans += a;
                heap.push((b, 0));
            }
            _ => break,
        }
    }

    println!("{}", ans);
}

mod utils {
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }
    pub(crate) use debug;
}
