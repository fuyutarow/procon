use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::mem::swap;

fn main() {
    input! {
        N: usize,
        graph: [ Chars; N],
    }

    for r in 0..N {
        for c in 0..N {
            if graph[r][c] != '#' {
                continue;
            }

            'a: for (ii, jj) in &[(0, 1), (1, -1), (1, 0), (1, 1)] {
                let mut life = 2;
                let mut i = r as i32;
                let mut j = c as i32;
                let mut cnt = 1;
                'b: while life >= 0 {
                    i += ii;
                    j += jj;
                    cnt += 1;

                    // dbg!((r, c, i, j, cnt));

                    if !(0 <= i && i < N as i32 && 0 <= j && j < N as i32) {
                        continue 'a;
                    }

                    if graph[i as usize][j as usize] == '.' {
                        life -= 1;
                        continue 'b;
                    }

                    if cnt >= 6 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
