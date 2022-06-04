use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    A.sort();

    let mut res = 0;
    let mut i = 0;

    while i < A.len() && A[i] == res {
        while i + 1 < A.len() && A[i] == A[i + 1] {
            i += 1;
        }

        i += 1;
        res += 1;
    }
    println!("{}", &res);
}
