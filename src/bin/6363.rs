struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut ret = vec![];

        let mut map = HashMap::<i32, i32>::new();
        for num in nums {
            *map.entry(num).or_default() += 1;
        }
        let mut num_remain = 1;
        while num_remain != 0 {
            num_remain = 0;
            ret.push(vec![]);
            let arr = ret.last_mut().unwrap();

            for (k, v) in map.iter_mut() {
                if *v == 0 {
                    continue;
                }
                *v -= 1;
                if *v != 0 {
                    num_remain += 1;
                }
                arr.push(*k);
            }
        }

        ret
    }
}

fn main() {}
