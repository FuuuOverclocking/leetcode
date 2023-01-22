struct Solution {}

impl Solution {
    pub fn max_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                let mut val = grid[i][j];
                if i > 0 {
                    let (x, y) = (i - 1, j);
                    val = std::cmp::max(val, grid[x][y] + grid[i][j]);
                }
                if j > 0 {
                    let (x, y) = (i, j - 1);
                    val = std::cmp::max(val, grid[x][y] + grid[i][j]);
                }
                grid[i][j] = val;
            }
        }

        grid[m - 1][n - 1]
    }
}

fn main() {}
