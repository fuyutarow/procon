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
use std::cmp::{max as _max, min as _min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
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
        S: Chars,
    }

    let n_comb = |n: u64| -> u64 { n * (n + 1) / 2 };

    let mut runlength = vec![(S[0], 1)];

    for (prev, curr) in S.iter().tuple_windows() {
        if prev == curr {
            let last = runlength.last_mut().unwrap();
            last.1 += 1;
        } else {
            runlength.push((*curr, 1));
        }
    }

    let res: u64 = n_comb(N as u64)
        - runlength
            .into_iter()
            .map(|(_, n)| n_comb(n as u64))
            .sum::<u64>();

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
