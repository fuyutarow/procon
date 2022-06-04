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
        N: usize,
        mut H: [i32; N],
    }

    let mut ret = 0;

    for i in 0..N {
        while H[i] > 0 {
            for j in i..N {
                if H[j] > 0 {
                    H[j] -= 1;
                    continue;
                } else {
                    break;
                }
            }
            ret += 1;
        }
    }

    println!("{}", ret);
}
