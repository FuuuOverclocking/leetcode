struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut ans = vec![];
        nums.sort_unstable();

        for i in 0..(nums.len() - 2) {
            if nums[i] > 0 {
                break;
            }
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let k = nums[i];
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            loop {
                let sum = nums[l] + nums[r];
                if sum == -k {
                    ans.push(vec![k, nums[l], nums[r]]);
                    l += 1;
                    loop {
                        if l >= r {
                            break;
                        }
                        if nums[l] == nums[l - 1] {
                            l += 1;
                        } else {
                            break;
                        }
                    }
                    r -= 1;
                    loop {
                        if l >= r {
                            break;
                        }
                        if nums[r] == nums[r + 1] {
                            r -= 1
                        } else {
                            break;
                        }
                    }
                } else if sum < -k {
                    l += 1;
                } else {
                    r -= 1;
                }
                if l >= r {
                    break;
                }
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
