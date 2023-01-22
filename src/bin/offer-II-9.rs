struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut product: i32 = 1;
        let mut ret = 0;

        let mut l = 0;
        for r in 0..nums.len() {
            product *= nums[r];
            while l <= r && product >= k {
                product /= nums[l];
                l += 1;
            }
            ret += r - l + 1;
        }

        ret as i32
    }
}

fn main() {
    // println!("{:?}", );
}
