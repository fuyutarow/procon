use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        N: i32,
    }

    fn paren(n: i32) -> Vec<String> {
        if n < 2 {
            return vec![String::from("()")];
        }

        let mut res = vec![];

        for s in paren(n) {
            res.push(format!("(){}", s));
            res.push(format!("({})", s));
            res.push(format!("{}()", s));
        }

        res
    };

    dbg!(paren(N));

    // for c in paren(N) {
    //     println!("{}", c);
    // }
}
