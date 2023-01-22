struct Solution {}

impl Solution {
    fn concat(a: i32, b: i32) -> i64 {
        let i = {
            if b == 0 {
                1
            } else {
                let mut tmp = b;
                let mut i = 0;
                while tmp != 0 {
                    i += 1;
                    tmp /= 10;
                }
                i
            }
        };
        return (a as i64) * 10_i64.pow(i) + b as i64;
    }

    pub fn min_number(mut nums: Vec<i32>) -> String {
        use std::cmp::Ordering;
        use std::fmt::Write;

        let mut result = String::new();

        nums.sort_by(|&a, &b| Solution::concat(a, b).cmp(&Solution::concat(b, a)));

        for n in nums {
            write!(&mut result, "{}", n);
        }
        result
    }
}

fn main() {}
