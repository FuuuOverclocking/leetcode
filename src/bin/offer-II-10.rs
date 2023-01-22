struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut ret = 0;
        let mut map = HashMap::with_capacity(nums.len());
        let mut sum = 0;

        map.insert(0, 1);
        
        for val in nums {
            sum += val;
            if let Some(&n) = map.get(&(sum - k)) {
                ret += n;
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}

fn main() {
    println!("{:?}", Solution::subarray_sum(vec![1, 1, 1], 2));
}
