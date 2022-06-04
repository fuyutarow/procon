use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        mut A: i32,
        mut B: i32,
        mut C: i32,
    }

    let mut cnt = 0;
    let mut seen = HashSet::new();
    while A % 2 == 0 && B % 2 == 0 && C % 2 == 0 {
        if seen.contains(&(A, B, C)) {
            println!("-1");
            return;
        }
        seen.insert((A, B, C));
        let a = A / 2;
        let b = B / 2;
        let c = C / 2;
        A = b + c;
        B = a + c;
        C = a + b;
        cnt += 1;
    }

    println!("{}", cnt);
}
