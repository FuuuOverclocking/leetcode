struct Solution {}

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        unsafe {
            result.set_len(nums.len());
        }
        let mut odd = 0usize;
        let mut even = nums.len() - 1;
        for n in nums {
            if n % 2 == 0 {
                result[even] = n;
                even -= 1;
            } else {
                result[odd] = n;
                odd += 1;
            }
        }
        return result;
    }
}

fn main() {}
