use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        n: usize,
        mut cs: Chars,
        q: usize,
        ts: [(usize, usize, usize); q]
    }
}
