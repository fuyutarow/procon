#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use ac_library_rs::pow_mod;
use itertools::Itertools;
use num::integer::gcd;
use petgraph::algo::kosaraju_scc as scc;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
const TAU: f64 = PI * 2.;

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ac_library_rs::{
    floor_sum, lcp_array, suffix_array, Dsu, MfGraph, Mod998244353, ModInt1000000007,
};
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, FenwickTree, MultiSet, Segtree},
    monoid::{Max, Min, Monoid, Product, Sum},
};
use utils::debug;

#[fastout]
fn main() {
    input! {
        N: u64,
        K: u64,
    }

    const MOD: u32 = 1_000_000_007;
    let mut res = ModInt1000000007::new(1u64);

    for i in (0..N).take(2) {
        res = res * (K - i);
    }

    let n = N.saturating_sub(2);
    let k = K.saturating_sub(2);
    res = 
    res = pow_mod(k as i64, n as i64, MOD) * res % MOD;

    println!("{}", res);
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
