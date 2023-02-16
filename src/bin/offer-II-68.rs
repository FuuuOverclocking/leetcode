struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        if target < nums[l] {
            return 0;
        }
        if target > nums[r] {
            return nums.len() as i32;
        }
        if nums.len() == 1 {
            return 0;
        }

        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                r = mid;
            } else if l == mid {
                return (l + 1) as i32;
            } else {
                l = mid;
            }
        }
        unreachable!()
    }
}

fn main() {}
