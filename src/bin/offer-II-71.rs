use rand::{thread_rng, Rng};
struct Solution {
    sum: Vec<i32>,
    total: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut sum = vec![0; w.len()];
        let mut total = 0;
        for i in 0..w.len() {
            total += w[i];
            sum[i] = total;
        }
        Self { sum, total }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = thread_rng();
        let target = rng.gen_range(0..self.total);
        let mut l = 0;
        let mut r = self.sum.len() - 1;
        while l < r {
            let mid = l + (r - l) / 2;
            if self.sum[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}

fn main() {}
