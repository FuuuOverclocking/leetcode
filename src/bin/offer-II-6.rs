struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        loop {
            let sum = numbers[l] + numbers[r];
            if sum == target {
                return vec![l as i32, r as i32];
            }
            if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
}

fn main() {
    // println!("{:?}", Solution::two_sum(numbers, target));
}
