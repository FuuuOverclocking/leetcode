struct Solution {}

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let map = {
            let mut map = HashMap::new();

            for (i, &v) in arr2.iter().enumerate() {
                map.insert(v, i);
            }
            map
        };

        arr1.sort_unstable_by(|a, b| {
            let a_in_arr2 = map.contains_key(a);
            let b_in_arr2 = map.contains_key(b);

            if a_in_arr2 && b_in_arr2 {
                map[a].cmp(&map[b])
            } else if a_in_arr2 {
                std::cmp::Ordering::Less
            } else if b_in_arr2 {
                std::cmp::Ordering::Greater
            } else {
                a.cmp(b)
            }
        });

        arr1
    }
}

fn main() {}
