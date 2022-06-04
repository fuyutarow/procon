use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        N: usize,
        mut S: Chars,
    }

    let mut cha2idx = HashMap::new();
    for (i, c) in "atcoder".chars().enumerate() {
        cha2idx.insert(c, i + 1);
    }

    let mut dp = vec![0; N + 1];
    dp[0] = 1;

    for c in S {
        if let Some(&idx) = cha2idx.get(&c) {
            dp[idx] = (dp[idx] + dp[idx - 1]);
            dbg!(&dp);
        }
    }

    dbg!(&dp);
}
