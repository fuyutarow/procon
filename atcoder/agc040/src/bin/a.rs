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

    let mut top = 0;
    let mut bottom = 0;

    let mut dp = vec![0; S.len() + 1];

    while S[bottom] == '<' {
        bottom += 1;
    }
    while S[top] == '>' {
        top += 1;
    }
}
