use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars;N],
    }

    let mut book = HashMap::new();

    for (i, username) in S.iter().enumerate() {
        if !book.contains_key(&username) {
            book.insert(username, i);
            println!("{}", i + 1);
        }
    }
}
