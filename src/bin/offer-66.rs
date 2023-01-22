struct Solution {}

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        if a.len() == 0 {
            return vec![];
        }
        if a.len() == 1 {
            return vec![1];
        }

        let n = a.len();
        let mut ans = vec![0; n];

        ans[0] = 1;
        for i in 1..n {
            ans[i] = ans[i - 1] * a[i - 1];
        }

        let mut r = 1;
        for i in (0..n).rev() {
            ans[i] = ans[i] * r;
            r *= a[i];
        }

        ans
    }
}

fn main() {
    for i in (0..10).rev() {
        println!("{}", i);
    }
}
