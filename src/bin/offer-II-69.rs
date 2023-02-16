struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        for i in 1..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                return i as i32;
            }
        }
        unreachable!()
    }
}

fn main() {}
