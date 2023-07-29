struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut sum = 0;
        let mut pre = vec![0; nums.len()];
        let mut post = vec![0; nums.len()];

        for i in 0..nums.len() {
            let k = nums[i];
            pre[i] = sum;
            sum += k;
        }
        sum = 0;
        for i in (0..nums.len()).rev() {
            let k = nums[i];
            post[i] = sum;
            sum += k;
        }
        for i in 0..nums.len() {
            if pre[i] == post[i] {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {
    // println!("{:?}", Solution);
}
