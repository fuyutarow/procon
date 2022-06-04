use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::vec;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; M],
    }

    let mut dp = vec![0; N + 1];
    dp[0] = 1;
    dp[1] = if A[0] == 1 { 0 } else { 1 };

    let mut A = VecDeque::from(A);

    for i in 2..=N {
        if let Some(&a) = A.front() {
            if i == a {
                A.pop_front();
                continue;
            }
        }
        dp[i] = (dp[i - 1] + dp[i - 2]) % 1_000_000_007;
    }

    println!("{}", dp[N]);
}
