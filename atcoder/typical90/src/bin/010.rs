use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        N: usize,
        CP: [(i32,i32); N],
        Q: usize,
        LR: [(usize,usize); Q],
    }

    let mut first_prefixsums = vec![0; N + 1];
    let mut second_prefixsums = vec![0; N + 1];

    let mut first_prefixsum = 0;
    let mut second_prefixsum = 0;
    for (i, (class, point)) in CP.iter().enumerate() {
        if *class == 1 {
            first_prefixsum += point;
        } else {
            second_prefixsum += point;
        }
        first_prefixsums[i + 1] = first_prefixsum;
        second_prefixsums[i + 1] = second_prefixsum;
    }

    for (left, right) in LR {
        let rr = first_prefixsums[right];
        let ll = first_prefixsums[left - 1];
        let first = rr - ll;
        let rr = second_prefixsums[right];
        let ll = second_prefixsums[left - 1];
        let second = rr - ll;

        println!("{} {}", first, second);
    }
}
