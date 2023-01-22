use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        let n = n as usize;

        let mut prev = vec![0_usize; 6 * n + 1];
        let mut curr = vec![0_usize; 6 * n + 1];

        for i in 1..=6 {
            prev[i] = 1;
        }

        for i in 2..=n {
            let prev_min = i - 1;
            let prev_max = 6 * (i - 1);
            // let min = i;
            // let max = 6 * i;

            curr.fill(0);
            for j in 1..=6 {
                for k in prev_min..=prev_max {
                    curr[j + k] += prev[k];
                }
            }
            swap(&mut curr, &mut prev);
        }

        let mut ans = Vec::<f64>::with_capacity(5 * n + 1);
        let total = 6_u64.pow(n as u32) as f64;
        for i in n..=(6 * n) {
            ans.push(prev[i] as f64 / total);
        }
        ans
    }
}

fn main() {}
