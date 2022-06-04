use num::integer::gcd;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::i32::MAX as inf;
use std::mem::swap;
const TAU: f64 = PI * 2.;

fn dfs(tour: &[usize], graph: &[Vec<usize>], X: &[usize], root: usize) -> Vec<usize> {
    let mut ret = vec![X[root]];

    let children = graph[root];

    for child in children {
        let tour = dfs(tour, graph, X, child);
        ret.append(&mut tour)
    }

    ret.ret
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [i32; N],
        AB: [(Usize1, Usize1); N-1],
        VQ: [(i32, i32); Q],
    }
    dbg!((&AB));

    let mut graph = vec![vec![]; N];

    for (a, b) in AB {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut tour = vec![];

    dfs(&tour, &graph, 0);

    dbg!((&graph));
}
