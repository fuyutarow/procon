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
        M: usize,
        mut A: [i32; N+1],
        mut C: [i32; N+M+1],
    }

    A.reverse();
    C.reverse();

    let mut B = vec![0; M + 1];

    for i in 0..B.len() {
        B[i] = C[i] / A[0];

        for j in 0..A.len() {
            C[i + j] -= B[i] * A[j];
        }
    }

    B.reverse();
    println!("{}", B.iter().join(" "));
}
