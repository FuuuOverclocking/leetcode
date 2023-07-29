struct NumMatrix {
    mat: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 0..m {
            for j in 0..n {
                let l = if j == 0 { 0 } else { matrix[i][j - 1] };
                let t = if i == 0 { 0 } else { matrix[i - 1][j] };
                let lt = if i == 0 || j == 0 {
                    0
                } else {
                    matrix[i - 1][j - 1]
                };
                matrix[i][j] += l;
                matrix[i][j] += t;
                matrix[i][j] -= lt;
            }
        }

        Self { mat: matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;

        let l = if col1 == 0 {
            0
        } else {
            self.mat[row2][col1 - 1]
        };
        let t = if row1 == 0 {
            0
        } else {
            self.mat[row1 - 1][col2]
        };
        let lt = if row1 == 0 || col1 == 0 {
            0
        } else {
            self.mat[row1 - 1][col1 - 1]
        };

        self.mat[row2][col2] - l - t + lt
    }
}

fn main() {
    // println!("{:?}", Solution);
}
