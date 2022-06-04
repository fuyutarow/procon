use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        K: i32,
        A: [i32; N],
        B: [i32; N],
    }

    let s: i32 = A.iter().zip(B.iter()).map(|(a, b)| (a - b).abs()).sum();

    if K - s < 0 {
        println!("No");
    } else if (K - s) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
