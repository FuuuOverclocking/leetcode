struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut count = 0_i32;
        let mut map = HashMap::<i32, i32>::new();
        map.insert(0, -1);
        let mut max_len = 0;

        for (i, &k) in nums.iter().enumerate() {
            count += if k == 1 { 1 } else { -1 };
            if let Some(idx) = map.get(&count) {
                max_len = max_len.max(i as i32 - idx);
            } else {
                map.insert(count, i as i32);
            }
        }

        max_len
    }
}

fn main() {
    // println!("{:?}", Solution);
}
