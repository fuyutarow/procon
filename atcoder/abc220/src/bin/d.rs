use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
use std::i32::MAX as inf;
use std::mem::swap;
use std::vec;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    const MOD: usize = 998244353;

    let mut dp = vec![0; 10];
    dp[A[0]] = 1;

    for i in 1..N {
        let mut next = vec![0; 10];
        for j in 0..10 {
            let jj = (j + A[i]) % 10;

            next[jj] += dp[j] % MOD;

            let jj = (j * A[i]) % 10;
            next[jj] += dp[j] % MOD;
        }
        dp = next;
    }

    for e in dp {
        println!("{}", e % MOD);
    }
}
