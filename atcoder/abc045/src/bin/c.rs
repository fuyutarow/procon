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

    let N = S.len();

    // for bit in 0..(1 << N) {
    //     let sublist = (0..N).filter(|e| (bit & 1 << e) != 0);
    let mut res = 0;
    for bit in 0..(1 << N - 1) {
        let sublist = (0..N - 1)
            .filter(|e| (bit & 1 << e) != 0)
            .collect::<Vec<_>>();

        dbg!(&sublist);

        // for i in 0..sublist.len() {
        //     let start = sublist[i];
        //     let end = sublist[i + 1];
        //     dbg!(&S[start..end]);
        //     // res += S[st..idx + 1]
        //     //     .iter()
        //     //     .collect::<String>()
        //     //     .parse::<i32>()
        //     //     .unwrap();
        // }
    }

    println!("{}", res);
}
