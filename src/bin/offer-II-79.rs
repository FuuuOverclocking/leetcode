struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];

        ret.push(vec![]);
        ret.push(nums.clone()); // nums.len() >= 1

        let len = nums.len();

        fn work(
            ret: &mut Vec<Vec<i32>>,
            working: &mut Vec<i32>,
            nums: &[i32],
            need: usize,
        ) {
            if need == 1 {
                for &num in nums {
                    working.push(num);
                    ret.push(working.clone());
                    working.pop();
                }
                return;
            }

            for i in 0..=(nums.len() - need) {
                working.push(nums[i]);
                work(ret, working, &nums[i + 1..], need - 1);
                working.pop();
            }
        }

        for n in 1..=(len - 1) {
            let mut working = vec![];
            work(&mut ret, &mut working, &nums, n);
        }

        ret
    }
}

fn main() {
    Solution::subsets(vec![1, 2, 3]);
}
