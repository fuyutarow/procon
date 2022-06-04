use proconio::marker::*;
use proconio::*;

type Deque<T> = std::collections::VecDeque<T>;

fn main() {
    input! {
        n: usize,
        src: (Usize1, Usize1),
        dst: (Usize1, Usize1),
        s: [Bytes; n],
    }
    let inf = 1500 * 1500;
    let mut dp = vec![vec![inf; n]; n];
    dp[src.0][src.1] = 0;
    let mut deq = Deque::new();
    deq.push_back(src);
    while let Some((x, y)) = deq.pop_front() {
        let d = dp[x][y] + 1;
        for &(dx, dy) in [(1, 1), (1, !0), (!0, 1), (!0, !0)].iter() {
            let mut x = x + dx;
            let mut y = y + dy;
            while x < n && y < n && s[x][y] == b'.' && dp[x][y] >= d {
                if dp[x][y] > d {
                    dp[x][y] = d;
                    deq.push_back((x, y));
                }
                x += dx;
                y += dy;
            }
        }
    }
    let mut ans = dp[dst.0][dst.1];
    if ans == inf {
        ans = -1;
    }
    println!("{}", ans);
}
