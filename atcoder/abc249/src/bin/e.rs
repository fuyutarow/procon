#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use itertools::{Combinations, Itertools};
use num::integer::{binomial, gcd};
use petgraph::{algo::kosaraju_scc as scc, graph::NodeIndex, Directed, Graph};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max as _max, min as _min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
const TAU: f64 = PI * 2.;

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ac_library_rs::{floor_sum, Dsu, MfGraph, ModInt1000000007, ModInt998244353, StaticModInt};
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
        P: usize,
    }

    // let n_pattern = |n: usize, k| -> usize {
    //     binomial(
    //         n.saturating_sub(1),
    //         ((n.saturating_sub(1)) / 2).saturating_sub(1),
    //     )
    // };

    if N <= 2 {
        println!("{}", 0);
        return;
    }

    let max_k = |n: usize| -> usize { (n.saturating_sub(1) / 2).saturating_sub(1) };

    let mut ans = 26;
    for k in 0..max_k(N) {
        ans += (binomial(N.saturating_sub(1), k) % P) * (26i32 * 25i32.pow(k as u32)) % 2;
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
