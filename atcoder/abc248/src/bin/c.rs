#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use ac_library_rs::Mod998244353;
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
        M: usize,
        K: usize,
    }

    let mut ans = ModInt998244353::new(0u64);

    fn dfs(idx: usize, depth: usize, N: usize, M: usize, K: usize) -> Mod998244353 {
        debug!(idx, depth);
        if depth >= N {
            if K - idx <= M {
                ModInt998244353::new(1u64)
            } else {
                ModInt998244353::new(0u64)
            }
        } else {
            dfs(idx + 1, depth + 1, N, M, K)
        }
    }

    // dfs(0, 0, &mut ans, N, M, K);

    println!("{}", ans);
}

#[fastout]
fn _main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    let mut ans = ModInt998244353::new(0u64);

    fn dfs(idx: usize, depth: usize, ans: &mut ModInt998244353, N: usize, M: usize, K: usize) {
        debug!(idx, depth);
        if depth >= N {
            if K - idx <= M {
                *ans += 1;
            }
        } else {
            dfs(idx + 1, depth + 1, ans, N, M, K);
        }
    }

    dfs(0, 0, &mut ans, N, M, K);

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
