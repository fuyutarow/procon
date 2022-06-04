use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut mp = HashMap::new();

    for (i, a) in A.iter().enumerate() {
        mp.insert(i + 1, *a);
    }

    let mut val = *mp.get(&1).unwrap();
    mp.remove(&1);
    let mut res = 1;
    while val != 2 {
        if let Some(&v) = mp.get(&val) {
            mp.remove(&val);
            val = v;
            res += 1;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", res);
}
