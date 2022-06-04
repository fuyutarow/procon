use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
use std::i32::MAX as inf;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
    }

    let N = S.len() - 1;
    let mut res = 0;

    for (i, &c) in S.iter().enumerate() {
        res += if c == 'U' {
            (N - i) + i * 2
        } else {
            (N - i) * 2 + i
        };
    }

    println!("{}", &res);
}
