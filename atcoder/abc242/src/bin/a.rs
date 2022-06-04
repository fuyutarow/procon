use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let res = if x <= a {
        1.
    } else if b < x {
        0.
    } else {
        c as f32 / (b - a) as f32
    };

    println!("{}", res);
}
