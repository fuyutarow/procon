use itertools::{iproduct, izip, Itertools};
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
        mut N: u64,
        K: usize,
    }

    for _ in 0..K {
        let mut digit = 0;
        let mut i = 0;
        while N > 0 {
            digit += N % 10 * 8u64.pow(i);
            N /= 10;
            i += 1;
        }

        let mut nines = 0;
        let mut j = 0;

        while digit > 0 {
            let d = digit % 9;
            if d == 8 {
                nines += 5 * 10u64.pow(j);
            } else {
                nines += d * 10u64.pow(j);
            }
            digit /= 9;
            j += 1;
        }

        N = nines;
    }

    println!("{}", N);
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
