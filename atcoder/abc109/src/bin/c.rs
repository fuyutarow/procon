use itertools::Itertools;
use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        X: i32,
        mut A: [i32; N],
    }

    let res = A.iter().map(|a| (a - X).abs()).fold(0, |x, y| gcd(x, y));

    println!("{}", res);
}
