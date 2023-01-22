struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::_search(&nums, target)
    }
    fn _search(nums: &[i32], target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return if nums[0] == target { 1 } else { 0 };
        }
        let mid = nums.len() / 2;
        if nums[mid] > target {
            return Solution::_search(&nums[0..mid], target);
        } else if nums[mid] < target {
            return Solution::_search(&nums[(mid + 1)..], target);
        } else {
            return 1
                + Solution::_search(&nums[0..mid], target)
                + Solution::_search(&nums[(mid + 1)..], target);
        }
    }
}

fn main() {}
