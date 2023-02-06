use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        // val, occur
        let mut map = HashMap::<i32, i32>::new();

        for n in nums {
            *map.entry(n).or_default() += 1;
        }

        let mut arr: Vec<_> = map.into_iter().collect();
        let len = arr.len();

        quick_sort(arr.as_mut(), k, 0, len - 1);
        fn quick_sort(arr: &mut [(i32, i32)], k: usize, l: usize, r: usize) {
            let pivot = arr[l].1;
            let mut i = l + 1;
            let mut j = l + 1;

            while j != r + 1 {
                if arr[j].1 > pivot {
                    let tmp = arr[i];
                    arr[i] = arr[j];
                    arr[j] = tmp;
                    i += 1;
                }
                j += 1;
            }

            let tmp = arr[i - 1];
            arr[i - 1] = arr[l];
            arr[l] = tmp;
            if i - 1 == k {
                return;
            } else if i - 1 > k {
                quick_sort(arr, k, l, i - 2);
            } else {
                quick_sort(arr, k, i - 1, r);
            }
        }

        arr.truncate(k);
        arr.into_iter().map(|p| p.0).collect()
    }
}

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
}
