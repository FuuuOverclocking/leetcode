struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;

        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        let mut q = VecDeque::<usize>::with_capacity(k + 1);

        for i in 0..k {
            while let Some(&last) = q.back() {
                if nums[last] > nums[i] {
                    break;
                }
                q.pop_back();
            }
            q.push_back(i);
        }
        result.push(nums[*q.front().unwrap()]);

        for i in k..nums.len() {
            while let Some(&last) = q.back() {
                if nums[last] > nums[i] {
                    break;
                }
                q.pop_back();
            }
            q.push_back(i);

            while let Some(&first) = q.front() {
                if first > i - k {
                    break;
                }
                q.pop_front();
            }
            result.push(nums[*q.front().unwrap()]);
        }

        result
    }
}

fn main() {}
