use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        mut cs: Chars,
    }

    cs.sort();

    let s = cs.into_iter().collect::<String>();
    println!("{}", s);
}
