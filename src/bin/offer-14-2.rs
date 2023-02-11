struct Solution {}

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let mut res = 1_i64;
        let mut n = n as i64;
        while n > 4 {
            res *= 3;
            res %= 1000000007;
            n -= 3;
        }

        res = (res * n) % 1000000007;
        res as i32
    }
}

fn main() {}
