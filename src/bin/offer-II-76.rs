struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        pub fn find_kth_largest(
            nums: &mut Vec<i32>,
            k: usize,
            l: usize,
            r: usize,
        ) -> i32 {
            let pivot = nums[l];
            let mut next_pos = l + 1;

            for i in (l + 1)..=r {
                if nums[i] > pivot {
                    let tmp = nums[next_pos];
                    nums[next_pos] = nums[i];
                    nums[i] = tmp;
                    next_pos += 1;
                }
            }

            let tmp = nums[next_pos - 1];
            nums[next_pos - 1] = nums[l];
            nums[l] = tmp;

            if next_pos == k {
                pivot
            } else if next_pos < k {
                find_kth_largest(nums, k, next_pos, r)
            } else {
                find_kth_largest(nums, k, l, next_pos - 2)
            }
        }

        let r = nums.len() - 1;
        find_kth_largest(&mut nums, k as usize, 0, r)
    }
}
fn main() {}
