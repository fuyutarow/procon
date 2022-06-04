use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        (a1, a2): (i32, i32),
        (b1, b2): (i32, i32),
    }

    let start = max(a1, b1);
    let end = min(a2, b2);

    let res = max(0, end - start);

    println!("{}", res);
}
