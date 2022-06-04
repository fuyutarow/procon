use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::marker::Chars;
use proconio::{fastout, input};
use rand_distr::Uniform;
use std::collections::HashMap;
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        H: i32,
        W: i32,
        Q: usize,
    }

    let mut graph = vec![vec![0; W as usize]; H as usize];
    let mut dsu = UnionFind::<usize>::new(((W + 1) * (H + 1)) as usize);

    for _ in 0..Q {
        input! { query_type: usize };

        if query_type == 1 {
            input! {
               r: i32,
               c: i32,
            }
            let r = r - 1;
            let c = c - 1;

            graph[r as usize][c as usize] = 1;
            for (rr, cc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if 0 <= rr
                    && rr < H as i32
                    && 0 <= cc
                    && cc < W as i32
                    && graph[rr as usize][cc as usize] == 1
                {
                    dsu.union((r * W + c) as usize, (rr * W + cc) as usize);
                }
            }
        } else {
            input! {
                r1: i32,
                c1: i32,
                r2: i32,
                c2: i32,
            }

            if dsu.equiv((r1 * W + c1) as usize, (r2 * W + c2) as usize) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
