#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

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
use ac_library_rs::{Dsu, MfGraph};
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, FenwickTree, MultiSet},
};
use utils::debug;

// #[fastout]
fn main() {
    input! {
        N: usize,
        Qn: usize,
        A: [usize; N],
        Q: [(usize, usize, usize); Qn],
    }

    let mut bit = FenwickTree::from(A);

    for q in Q {
        match q {
            (0, p, x) => {
                bit.add(p, x);
            }
            (1, l, r) => {
                let res = bit.sum(l, r);
                println!("{}", res);
            }
            _ => unreachable!(),
        }
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
