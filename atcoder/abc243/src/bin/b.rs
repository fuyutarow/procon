use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut cnt = 0;
    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    for (a, b) in aa.iter().zip(bb.iter()) {
        a_set.insert(a);
        b_set.insert(b);

        if a == b {
            cnt += 1;
        }
    }

    let n_ab_set = a_set.intersection(&b_set).count();

    println!("{}", cnt);
    println!("{}", n_ab_set - cnt);
}
