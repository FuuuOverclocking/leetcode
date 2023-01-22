struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() <= 2 {
            panic!("too few");
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        
        loop {
            let sum = nums[l] + nums[r];
            if sum == target {
                return vec![nums[l], nums[r]];
            }
            if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
}

fn main() {}
