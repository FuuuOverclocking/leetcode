use std::ops::Mul;

struct Solution {}

const MOD: i64 = 1000000007;

#[derive(Clone, Copy)]
struct Mat([[i32; 2]; 2]);

impl Mul for Mat {
    type Output = Mat;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Mat([[0; 2]; 2]);
        let a = self.0;
        let b = rhs.0;
        res.0[0][0] =
            ((a[0][0] as i64 * b[0][0] as i64 + a[0][1] as i64 * b[1][0] as i64) % MOD) as i32;
        res.0[0][1] =
            ((a[0][0] as i64 * b[0][1] as i64 + a[0][1] as i64 * b[1][1] as i64) % MOD) as i32;
        res.0[1][0] =
            ((a[1][0] as i64 * b[0][0] as i64 + a[1][1] as i64 * b[1][0] as i64) % MOD) as i32;
        res.0[1][1] =
            ((a[1][0] as i64 * b[0][1] as i64 + a[1][1] as i64 * b[1][1] as i64) % MOD) as i32;
        res
    }
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as u32;
        if n < 2 {
            return n as i32;
        }
        let mat = Solution::pow(Mat([[1, 1], [1, 0]]), n - 1);
        mat.0[0][0]
    }

    pub fn pow(mut mat: Mat, mut n: u32) -> Mat {
        let mut res = Mat([[1, 0], [0, 1]]);
        while n > 0 {
            if n & 1 == 1 {
                res = res * mat;
            }
            mat = mat * mat;
            n >>= 1;
        }
        res
    }
}

fn main() {
    println!("{}", Solution::fib(7));

    for i in 0..10 {
        println!("{}", Solution::fib(i));
    }
}
