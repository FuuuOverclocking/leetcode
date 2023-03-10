struct Solution {}

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let mut l = 0_usize;
        let mut r = numbers.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            if numbers[mid] > numbers[r] {
                l = mid + 1;
            } else if numbers[mid] < numbers[r] {
                r = mid;
            } else {
                r -= 1;
            }
        }

        numbers[l]
    }
}

fn main() {}
