use itertools::Itertools;
use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }

    let common = vec![A, B, C].iter().fold(0, |x, &y| gcd(x, y));

    let res: usize = vec![A, B, C].iter().map(|&a| a / common - 1).sum();

    println!("{}", res);
}
