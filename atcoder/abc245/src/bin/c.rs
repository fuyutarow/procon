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
        K: i32,
        A:[i32;N],
        B:[i32;N],
    }

    let mut prev = vec![A[0], B[0]];

    for i in 1..N {
        let mut curr = vec![];
        for p in &prev {
            if (*p - A[i]).abs() <= K {
                curr.push(A[i]);
            }
            if (*p - B[i]).abs() <= K {
                curr.push(B[i]);
            }
        }
        if curr.is_empty() {
            println!("No");
            return;
        }
        prev = curr;
    }
    println!("Yes");
}
