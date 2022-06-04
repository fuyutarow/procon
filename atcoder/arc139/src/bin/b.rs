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
        T: usize,
    }

    for _ in 0..T {
        input! {
            mut N: u64,
            A: u64,
            B: u64,
            X: u64,
            Y: u64,
            Z: u64,
        }

        let xp = 1f64 / X as f64;
        let yp = A as f64 / Y as f64;
        let zp = B as f64 / Z as f64;

        if xp >= yp && xp >= zp {
            println!("{}", N * X);
        } else if yp >= zp {
            let mut ans = (N / A) * Y;
            N %= A;

            if zp > xp {
                ans += (N / B) * Z;
                N %= B;
            }

            ans += N * X;

            println!("{}", ans);
        } else {
            let mut ans = (N / B) * Z;
            N %= B;

            if yp > xp {
                ans += (N / A) * Y;
                N %= A;
            }

            ans += N * X;
            println!("{}", ans);
        };
    }
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
