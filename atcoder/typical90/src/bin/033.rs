#![allow(
    dead_code,
    unused_imports,
    unused_macros,
    non_snake_case,
    non_camel_case_types
)]

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
    }

    if min(H, W) == 1 {
        println!("{}", H * W);
    } else {
        let res = ((H + 1) / 2) * ((W + 1) / 2);
        println!("{}", res);
    }
}
