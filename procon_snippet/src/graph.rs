use cargo_snippet::snippet;

#[snippet("dfs", prefix = "#[derive(Debug)]")]
struct Dfs<'a> {
    graph: &'a Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

#[snippet("dfs")]
impl Dfs<'_> {
    fn dfs(&mut self, i: isize, j: isize) -> bool {
        // 境界条件
        if i < 0 || self.h as isize <= i || j < 0 || self.w as isize <= j {
            false
        } else {
            let visited = &mut self.visited[i as usize][j as usize];
            match self.graph[i as usize][j as usize] {
                'g' => true,
                '.' | 's' if !*visited => {
                    *visited = true;
                    self.dfs(i - 1, j)
                        || self.dfs(i, j - 1)
                        || self.dfs(i, j + 1)
                        || self.dfs(i + 1, j)
                }
                _ => false,
            }
        }
    }
}
