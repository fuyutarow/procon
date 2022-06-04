#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ac_library_rs::{floor_sum, Dsu, MfGraph};
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, FenwickTree, MultiSet, Segtree},
    impl_monoid,
    monoid::{Max, Min, Monoid, Product, Sum},
};
use utils::debug;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [u64; N],
    }

    impl_monoid!(Max, u64, u64::min_value(), |&x, &y| max(x, y));

    let mut segtree = Segtree::<Max>::from(A);

    for _ in 0..Q {
        input! { q: usize }

        if q == 1 {
            input! { x: Usize1, v: u64 }
            segtree.insert(x, v);
        } else if q == 2 {
            input! { l: Usize1, r: usize }
            let val = segtree.fold(l..r);
            println!("{}", val);
        } else {
            input! { x: Usize1, v: u64 }
            let j = segtree.max_right(x.., |&a| a < v);
            println!("{}", j + 1);
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
