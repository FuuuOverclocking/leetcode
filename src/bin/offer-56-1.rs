struct Solution {}

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut a = 0;
        for n in &nums {
            a ^= n;
        }
        // a = x ^ y

        let mut l = 1;
        loop {
            if l & a != 0 {
                break;
            }
            l <<= 1;
        }

        let mut x = 0;
        let mut y = 0;
        for n in &nums {
            if n & l == 0 {
                x ^= n;
            } else {
                y ^= n;
            }
        }

        return vec![x, y];
    }
}

fn main() {}
