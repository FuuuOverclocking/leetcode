struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mut mid = l + (r - l) / 2;
            if mid % 2 == 1 {
                mid -= 1;
            }
            if nums[mid] == nums[mid + 1] {
                l = mid + 2;
            } else {
                r = mid;
            }
        }
        nums[l]
    }
}

fn main() {}
