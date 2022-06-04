use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::mem::swap;
use std::vec;
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [(i32, i32); N-1],
    }

    let mut tree = HashMap::new();

    for (a, b) in &A {
        let v = tree.entry(a).or_insert(Vec::<i32>::new());
        v.push(*b);

        let v = tree.entry(b).or_insert(Vec::<i32>::new());
        v.push(*a);
    }

    let mut q = VecDeque::from(vec![(0, A[0].0)]);

    let mut max_sofar = 0;

    let mut book = HashMap::new();

    while let Some((depth, curr)) = q.pop_front() {
        let vv = book.entry(curr).or_insert(0);
        *vv = *vec![*vv, depth].iter().max().unwrap();

        if let Some(vs) = tree.get(&curr) {
            for v in vs {
                q.push_back((depth + 1, *v));
            }

            max_sofar = *vec![max_sofar, depth].iter().max().unwrap();
        }
    }

    println!("{}", &max_sofar);
}
