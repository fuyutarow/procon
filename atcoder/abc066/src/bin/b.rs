use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
    }

    for end in (0..S.len() - 1).rev() {
        let mut slow = 0;
        let mut fast = end;

        while slow < fast && S[slow] == S[fast] {
            slow += 1;
            fast -= 1;
        }
        if slow + 1 == fast {
            println!("{}", end);
            return;
        }
    }
}
