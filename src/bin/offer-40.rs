struct Solution {}

impl Solution {
    /// arr.len() >= k >= 0
    pub fn get_least_numbers(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }

        let k: usize = k as usize;
        Self::_get_least_numbers(&mut arr, k);
        return arr[0..k].into();
    }

    /// arr.len() >= k > 0
    fn _get_least_numbers(arr: &mut [i32], k: usize) {
        // arr[0] is pivot
        // arr[1..idx] < pivot
        // arr[idx..] >= pivot
        let mut idx = 1;
        let pivot = arr[0];

        for i in 1..arr.len() {
            if pivot > arr[i] {
                arr.swap(i, idx);
                idx += 1;
            }
        }

        arr.swap(0, idx - 1);

        // arr[0..(idx-1)] < pivot
        // arr[idx - 1] is pivot
        // arr[idx..] >= pivot

        if idx == k {
            return;
        }
        if idx > k {
            Self::_get_least_numbers(arr, k);
        } else {
            Self::_get_least_numbers(&mut arr[idx..], k - idx);
        }
    }
}

fn main() {
    Solution::get_least_numbers(vec![0, 0, 1, 2, 4, 2, 2, 3, 1, 4], 8);
}
