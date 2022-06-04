use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
    }

    if min(H, W) == 1 {
        println!("{}", 1);
    } else {
        println!("{}", (H * W + 1) / 2);
    }
}
