struct Solution {}

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        let first_removed = (m - 1) % n;
        if n == 2 {
            return 1 - first_removed;
        }

        let sub = Solution::last_remaining(n - 1, m);
        (first_removed + sub + 1) % n
    }
}

fn main() {}
