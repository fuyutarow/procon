use itertools::Itertools;
use num::integer::gcd;
use petgraph::algo::kosaraju_scc as scc;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};
use proconio::marker::{Chars, Usize1};
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
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M],
    }

    let graph = Graph::<NodeIndex<usize>, i16, Directed, usize>::from_edges(&UV);

    let comps = scc(&graph);

    let mut node_id2comp_id = HashMap::new();
    for (comp_id, comp) in comps.iter().enumerate() {
        for node in comp {
            node_id2comp_id.insert(node.index(), comp_id);
        }
    }

    let mut dp = vec![false; comps.len()];
    let mut ans = 0;

    for (comp_id, comp) in comps.iter().enumerate() {
        if comp.len() == 1 {
            for v in graph.neighbors(comp[0]) {
                dp[comp_id] |= dp[node_id2comp_id[&v.index()]];
            }
        } else {
            dp[comp_id] = true;
        }

        if dp[comp_id] {
            ans += comp.len();
        }
    }

    println!("{}", ans);
}
