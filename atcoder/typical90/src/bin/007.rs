use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::min;
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [i32; N],
        Q: usize,
        mut B: [i32; Q],
    }

    A.sort();
    for b in &B {
        let cost = || {
            let mut lo: i32 = -1;
            let mut hi = A.len() as i32 + 1;
            while lo + 1 < hi {
                let mid = (lo + hi) / 2;
                if A[mid as usize] >= *b {
                    hi = mid as i32;
                } else {
                    lo = mid as i32;
                }
            }

            if lo < 0 {
                A[hi as usize] - b
            } else if hi >= A.len() as i32 {
                b - A[lo as usize]
            } else {
                min(A[hi as usize] - b, b - A[lo as usize])
            }
        };
        println!("{}", cost());
    }
}
