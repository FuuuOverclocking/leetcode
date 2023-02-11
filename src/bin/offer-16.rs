struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n == 1 {
            return x;
        }

        if n == -1 {
            return 1.0 / x;
        }

        let mut res = Self::my_pow(x, n / 2);
        res *= res;
        if n % 2 != 0 {
            if n > 0 {
                res *= x;
            } else {
                res *= 1.0 / x;
            }
        }

        res
    }
}

fn main() {}
