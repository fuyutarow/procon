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
        mut S: Chars,
        A: [i32; N],
        ts: [(usize, usize, usize); N],
    }
}
