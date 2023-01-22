struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for n in &nums {
            let _a = a ^ n;
            let _b = (!b & a & n) | (b & !a & !n);
            a = _a;
            b = _b;
        }
        return a;
    }
}

fn main() {}
