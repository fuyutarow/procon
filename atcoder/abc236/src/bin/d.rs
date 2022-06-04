use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
use std::i32::MAX as inf;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![];

    for i in 0..2 * N - 1 {
        input! {
            AA: [ i32; 2*N -1 - i],
        }
        A.push(AA);
    }

    let reward = |(i, j): (usize, usize)| {
        let (i, j) = if i <= j { (i, j) } else { (j, i) };
        A[i][N - j]
    };

    

    dbg!(&A);
}
