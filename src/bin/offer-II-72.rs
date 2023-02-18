struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let k = x as f64;
        let mut x = x as f64;

        loop {
            let p = (x, x * x - k);
            let new_x = p.0 - p.1 / (2. * p.0);

            if (new_x - x).abs() < 1e-7 {
                break;
            }
            x = new_x;
        }
        x.floor() as i32
    }
}

fn main() {}
