use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        _n: usize,
        x: i64,
        s: Chars,
    }

    let mut i = x - 1;

    let mut depth = 0;

    while i > 1 << depth {
        depth += 1;
        i -= 1 << depth;
    }
    if i > 0 {
        depth += 1;
    }

    dbg!(i, depth);

    let mut idx = x - 1;
    if s.len() < 10 {
        for c in s {
            match c {
                'U' => {
                    let j = i - 1 << depth;
                    idx -= 1 << depth + j / 2 + (j + 1) % 2;
                    depth -= 1;
                }
                'L' => {
                    let j = i - 1 << depth;
                    idx += (1 << depth) + j / 2;
                    depth += 1;
                }
                'R' => {
                    let j = i - 1 << depth;
                    idx += (1 << depth) + j / 2 + 1;
                    depth += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{}", idx + 1);
}
