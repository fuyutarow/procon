use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[i32;W]; H],
    }

    let mut sums_by_col = vec![0; W];
    let mut sums_by_row = vec![0; H];

    for i in 0..H {
        sums_by_row[i] = A[i].iter().sum();
    }
    for j in 0..W {
        sums_by_col[j] = (0..H).map(|i| A[i][j]).sum();
    }

    for i in 0..H {
        let s = (0..W)
            .map(|j| {
                let res = sums_by_row[i] + sums_by_col[j] - A[i][j];
                res.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", s);
    }
}
