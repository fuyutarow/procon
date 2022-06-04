use itertools::Itertools;
use num::integer::gcd;
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
        K: usize,
        mut H: [i32; N],
    }

    H.sort();

    let mut slow = 0;
    let mut fast = K - 1;

    let mut min_sofar = std::i32::MAX;

    while fast < N {
        let diff = H[fast] - H[slow];
        min_sofar = min(min_sofar, diff);

        slow += 1;
        fast += 1;
    }

    println!("{}", min_sofar);
}
