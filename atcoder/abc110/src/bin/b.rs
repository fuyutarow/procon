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
        mut S: Chars,
        A: [i32; N],
        ts: [(usize, usize, usize); N],
    }
}
