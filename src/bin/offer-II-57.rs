struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        k: i32,
        t: i32,
    ) -> bool {
        let k = k as usize;
        let t = t as i64;
        let mut map = HashMap::new();

        fn get_bucket_id(x: i64, width: i64) -> i32 {
            fn div_floor(a: i64, rhs: i64) -> i64 {
                let d = a / rhs;
                let r = a % rhs;
                if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
                    d - 1
                } else {
                    d
                }
            }
            div_floor(x, width) as i32
        }

        for (i, &x) in nums.iter().enumerate() {
            let x = x as i64;
            let id = get_bucket_id(x, t + 1);
            if map.contains_key(&id) {
                return true;
            }
            if let Some(val) = map.get(&(id - 1)) {
                if (x - val).abs() <= t {
                    return true;
                }
            }
            if let Some(val) = map.get(&(id + 1)) {
                if (x - val).abs() <= t {
                    return true;
                }
            }
            map.insert(id, x);
            if i >= k {
                map.remove(&get_bucket_id(nums[i - k] as i64, t as i64 + 1));
            }
        }

        false
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::contains_nearby_almost_duplicate(
            vec![2147483647, -1, 2147483647],
            1,
            2147483647
        )
    );
}
