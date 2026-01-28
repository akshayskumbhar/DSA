impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let w = grid[0].len();
        let mut grid = grid.concat();
        let n = grid.len();
        let k = k as usize;

        let mut d = vec![i32::MAX / 2; n];
        d[0] = 0;

        let mut v: Vec<usize> = (0..n).collect();
        v.sort_by_key(|&i| -grid[i]);

        for a in 0..n {
            for i in 0..n {
                if i >= w {
                    d[i] = d[i].min(d[i - w] + grid[i]);
                }
                if i % w > 0 {
                    d[i] = d[i].min(d[i - 1] + grid[i]);
                }
            }

            if a == k {
                break;
            }

            let mut m = i32::MAX;
            let mut s = 0;

            for i in 0..n {
                if grid[v[i]] != grid[v[s]] {
                    for &j in &v[s..i] {
                        d[j] = m;
                    }
                    s = i;
                }
                m = m.min(d[v[i]]);
            }

            for &j in &v[s..n] {
                d[j] = m;
            }
        }

        d[n - 1]
    }
}
