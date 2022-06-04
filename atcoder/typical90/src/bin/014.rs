use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [i64; N],
        mut B: [i64; N],
    }

    A.sort();
    B.sort();

    let mut res = 0;

    for (a, b) in A.iter().zip(B.iter()) {
        res += (a - b).abs();
    }

    println!("{}", res);
}
