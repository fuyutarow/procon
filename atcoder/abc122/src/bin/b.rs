use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
    }

    let mut res = 0;
    let mut curr = 0;

    let atcg = "ATCG";
    for c in S {
        if atcg.contains(c) {
            curr += 1;
            res = max(res, curr);
        } else {
            curr = 0;
        }
    }
    println!("{}", &res);
}
