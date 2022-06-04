use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        A: f32,
        B: f32,
    }

    let r = ((B - 1.) / (A - 1.)).ceil();
    println!("{}", r);
}
