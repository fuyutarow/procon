use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        n: usize,
        xx: [(usize, usize); n],
        s: Chars,
    }

    let mut book_r = HashMap::new();
    let mut book_l = HashMap::new();

    for ((x, y), c) in xx.iter().zip(s.into_iter()) {
        if c == 'L' {
            let v = book_l.entry(y).or_insert(Vec::<usize>::new());
            v.push(x.to_owned());
        } else {
            let v = book_r.entry(y).or_insert(Vec::<usize>::new());
            v.push(x.to_owned());
        }
    }

    for (y, xlist_l) in book_l {
        if let Some(xlist_r) = book_r.get(y) {
            if xlist_r.iter().min() < xlist_l.iter().max() {
                println!("Yes");
                return;
            }
        } else {
            continue;
        }
    }

    println!("No");
}
