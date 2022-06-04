use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
    }

    let mut slow = 0;
    let mut res = 0;

    for (fast, &char) in S.iter().enumerate() {
        if char == 'W' {
            res += fast - slow;
            slow += 1;
        }
    }

    println!("{}", &res);
}
