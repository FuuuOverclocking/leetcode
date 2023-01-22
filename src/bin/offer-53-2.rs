struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if nums[0] != 0 {
            return 0;
        }

        let mut l = 0; // nums[l] == l
        let mut r = n; // nums[r] == r+1

        loop {
            if l + 1 == r {
                return r as i32;
            }

            let mid = (l + r) / 2;
            if nums[mid] == mid as i32 {
                l = mid;
            } else {
                r = mid;
            }
        }
    }
}

fn main() {}
