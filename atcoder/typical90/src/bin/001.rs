use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: i32,
        L: i32,
        K: i32,
        A: [i32; N],
    }

    let mut left: i32 = -1;
    let mut right: i32 = L + 1;

    while left + 1 < right {
        let mid = (left + right) / 2;

        let is_valid = || {
            let mut cnt = 0;
            let mut pre = 0;

            for a in &A {
                if a - pre >= mid && L - a >= mid {
                    cnt += 1;
                    pre = *a;
                }
            }
            cnt >= K
        };

        if is_valid() {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
