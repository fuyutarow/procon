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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
use std::{
    cmp::{max as _max, min as _min, Reverse},
    vec,
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
        ST: [(String, String); N],
    }

    let mut counter = ST
        .iter()
        .cloned()
        .map(|st| vec![st.0, st.1])
        .flatten()
        .collect::<Counter<_>>();

    let ans = ST.iter().all(|(s, t)| {
        *counter.get_mut(s).unwrap() -= 1;
        *counter.get_mut(t).unwrap() -= 1;

        // debug!( counter, s, counter.contains_key(s), t, counter.contains_key(t));

        let ok = counter.get(s).unwrap_or(&0).to_owned() == 0
            || counter.get(t).unwrap_or(&0).to_owned() == 0;
        *counter.get_mut(s).unwrap() += 1;
        *counter.get_mut(t).unwrap() += 1;
        debug!(counter, ok, s, counter.contains_key(s), counter.get(s));

        ok
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
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
