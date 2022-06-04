use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::i32::MAX as inf;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut N: i32,
        (A, B, C): (i32, i32, i32),
    }

    let mut res = inf;
    for a in 0..=min(9999, N / A) {
        let N = N - a * A;
        for b in 0..=min(9999 - a, N / B) {
            let N = N - b * B;

            if N % C == 0 {
                let c = N / C;
                res = min(res, a + b + c);
            }
        }
    }

    println!("{}", res);
}
