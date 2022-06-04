use itertools::Itertools;
use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
use std::i32::MAX as inf;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
     mut   X: [i32; M],
    }

    X.sort();
    let mut dX = vec![];
    for i in 0..M - 1 {
        let d = X[i + 1] - X[i];
        dX.push(d);
    }

    dX.sort_by_key(|&e| Reverse(e));

    if N - 1 < dX.len() {
        let res: i32 = dX[N - 1..].iter().sum();
        println!("{}", res);
    } else {
        println!("{}", 0);
    }
}
