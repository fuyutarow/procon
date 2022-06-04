use itertools::Itertools;
use num::integer::{gcd, lcm};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        A: u64,
        B: u64,
    }

    let g = gcd(A, B);

    let res = (A / g).saturating_mul(B);

    if res > 10u64.pow(18) {
        println!("Large");
    } else {
        println!("{}", res);
    }
}
