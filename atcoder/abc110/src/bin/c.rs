use itertools::Itertools;
use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
        mut T: Chars,
    }

    let mut book1 = HashMap::new();
    let mut book2 = HashMap::new();

    for (i, (s, t)) in S.iter().zip(T.iter()).enumerate() {
        let pos1 = book1.entry(s).or_insert(i);
        let pos2 = book2.entry(t).or_insert(i);

        if pos1 == pos2 {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
