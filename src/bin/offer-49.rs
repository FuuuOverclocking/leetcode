use std::cmp;

struct Solution {}
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![0; n];
        arr[0] = 1;

        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;

        for i in 1..n {
            let n2 = arr[p2] * 2;
            let n3 = arr[p3] * 3;
            let n5 = arr[p5] * 5;

            arr[i] = cmp::min(cmp::min(n2, n3), n5);
            if arr[i] == n2 {
                p2 += 1;
            }
            if arr[i] == n3 {
                p3 += 1;
            }
            if arr[i] == n5 {
                p5 += 1;
            }
        }

        arr[n - 1]
    }
}

fn main() {
    Solution::nth_ugly_number(10);
}
