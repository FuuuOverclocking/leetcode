struct Solution {}

impl Solution {
    pub fn translate_num(mut num: i32) -> i32 {
        let mut nums = vec![];
        if num == 0 {
            nums.push(0);
        } else {
            while num != 0 {
                nums.push(num % 10);
                num /= 10;
            }
            nums.reverse();
        }

        return Self::work(&nums);
    }
    fn work(nums: &[i32]) -> i32 {
        if nums.len() <= 1 {
            return 1;
        }
        if nums[0] == 0 || nums[0] * 10 + nums[1] > 25 {
            return Self::work(&nums[1..]);
        }
        return Self::work(&nums[1..]) + Self::work(&nums[2..]);
    }
}
fn main() {}
