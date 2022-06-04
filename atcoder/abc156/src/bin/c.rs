use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: i32,
        X: [i32; N],
    }

    // let mut res = 1e9 as i32;
    let mut res = 2e30 as i32;

    for p in 1..100 {
        let cost = X.iter().map(|x| (x - p).pow(2)).sum();

        res = min(res, cost);
    }

    println!("{}", res);
}
