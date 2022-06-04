use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::i32::MAX as inf;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [[usize; N]; N],
        M: usize,
        XY: [(usize, usize); M],
    }

    let mut ok = vec![vec![true; N]; N];
    for (x, y) in XY {
        ok[x - 1][y - 1] = false;
        ok[y - 1][x - 1] = false;
    }

    let mut res = inf;
    for perm in (0..N).permutations(N) {
        if perm.windows(2).all(|p| ok[p[0]][p[1]]) {
            let r: usize = perm.iter().enumerate().map(|(i, &j)| A[j][i]).sum();
            res = min(res, r as i32);
        }
    }
    if res == inf {
        res = -1;
    }

    println!("{}", res);
}
