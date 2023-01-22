struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for c in nums {
            let tmp1 = b & c | a & !b & !c;
            let tmp2 = b & !c | !a & !b & c;
            a = tmp1;
            b = tmp2;
        }
        b
    }
}

fn main() {
    // println!("{:?}", Solution::count_bits(10));
}
