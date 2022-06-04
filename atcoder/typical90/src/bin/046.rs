use itertools::{iproduct, Itertools};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use acl_maxflow::MfGraph;
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use alds_lib::{
    chmax, chmin,
    collections::{Counter, MultiSet},
};
use utils::debug;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [i32; N],
        mut B: [i32; N],
        mut C: [i32; N],
    }

    let A = A.iter().map(|a| a % 46).collect::<Counter<_>>();
    let B = B.iter().map(|a| a % 46).collect::<Counter<_>>();
    let C = C.iter().map(|a| a % 46).collect::<Counter<_>>();

    let mut cnt = 0;
    for ((a, an), (b, bn), (c, cn)) in iproduct!(&A, &B, &C) {
        if (a + b + c) % 46 == 0 {
            cnt += an * bn * cn;
        }
    }

    println!("{}", cnt);
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
