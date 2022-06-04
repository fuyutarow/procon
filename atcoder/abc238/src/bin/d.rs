use num::integer::gcd;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
const TAU: f64 = PI * 2.;

#[fastout]
fn main() {
    input! {
        T: usize,
        AS: [(u64, u64); T],
    }

    fn solve(and: u64, sum: u64) -> bool {
        // sum - and >= 0
        // and AND (sum-and) == and
        sum >= and && and & (sum - and) == and
    }

    for (and, sum) in AS {
        if solve(and, sum) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
