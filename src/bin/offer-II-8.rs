struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut find = false;
        let mut min = i32::MAX;

        let mut l = 0;
        let mut r = 0;

        let mut sum = nums[0];
        loop {
            if sum >= target {
                find = true;
                min = min.min((r - l + 1) as i32);

                if l == nums.len() - 1 {
                    break;
                }
                sum -= nums[l];
                l += 1;
                if l > r {
                    r += 1;
                    sum += nums[r];
                }
            } else {
                r += 1;
                if r == nums.len() {
                    break;
                }
                sum += nums[r];
            }
        }

        if find {
            min
        } else {
            0
        }
    }
}

fn main() {
    // println!("{:?}", );
}
