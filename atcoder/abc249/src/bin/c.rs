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
        K: usize,
        // Sn: [String; N],
        Sn: [Chars; N],
    }

    let mut max_sofar = 0;
    for bit in 0..1 << N {
        let idxs = (0..N).filter(|i| bit & 1 << i > 0).collect::<Vec<_>>();

        let mut counter = Counter::<&char, usize>::new();
        debug!(idxs);
        for i in idxs {
            counter.extend(Sn[i].iter());
        }
        debug!(counter);

        let mut unique_chars = 0;
        for (k, v) in counter {
            if v == K {
                unique_chars += 1;
            }
        }

        chmax!(max_sofar, unique_chars);
    }

    println!("{}", max_sofar);
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
