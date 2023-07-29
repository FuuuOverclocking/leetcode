struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut min = 0;
        let mut sum = 0;
        for n in nums {
            sum += n;
            result = std::cmp::max(result, sum - min);
            min = std::cmp::min(min, sum);
        }

        return result;
    }
}

fn main() {}
