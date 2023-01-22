struct Solution {}

impl Solution {
    pub fn is_straight(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();

        let mut num_zero = 0;
        let mut prev = 0;

        let mut i = 0;
        while (i < nums.len()) {
            let n = nums[i];
            if n == 0 {
                num_zero += 1;
                i += 1;
                continue;
            }
            if prev == 0 {
                prev = n;
                i += 1;
                continue;
            }
            if prev + 1 != n {
                num_zero -= 1;
                if num_zero < 0 {
                    return false;
                }
                prev = prev + 1;
            } else {
                prev = n;
                i += 1;
            }
        }

        true
    }
}

fn main() {}
