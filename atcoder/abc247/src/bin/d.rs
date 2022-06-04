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
        Q: usize,
    }

    let mut runlength = VecDeque::<(u64, u64)>::new();

    for _ in 0..Q {
        input! { q: usize }

        if q == 1 {
            input! { x: u64, c: u64 }

            if let Some(mut last) = runlength.pop_back() {
                if last.0 == x {
                    last.1 += 1;
                    runlength.push_back(last);
                } else {
                    runlength.push_back(last);
                    runlength.push_back((x, c));
                }
            } else {
                runlength.push_back((x, c));
            }
        } else {
            input! { mut c: u64 }

            let mut ans = 0;
            while c > 0 {
                if let Some(mut head) = runlength.pop_front() {
                    if head.1 <= c {
                        ans += head.0 * head.1;
                        c -= head.1;
                    } else {
                        ans += head.0 * c;
                        head.1 -= c;
                        c = 0;
                        runlength.push_front(head);
                    }
                } else {
                    break;
                }
            }
            println!("{}", ans);
        }
        debug!(runlength);
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
