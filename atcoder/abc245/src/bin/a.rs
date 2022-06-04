use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
        D: i32,
    }

    let a = A * 60 + B;
    let b = C * 60 + D;

    if a <= b {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
