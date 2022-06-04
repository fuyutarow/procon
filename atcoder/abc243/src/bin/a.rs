use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let r = v % (a + b + c);

    if r < a {
        println!("F");
    } else if r < a + b {
        println!("M");
    } else {
        println!("T");
    }
}
