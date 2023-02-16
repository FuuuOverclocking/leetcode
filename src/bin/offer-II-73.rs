struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap();
        while l < r {
            let mid = l + (r - l) / 2;
            let mut hours = 0;
            for pile in piles.iter() {
                hours += pile / mid;
                if pile % mid != 0 {
                    hours += 1;
                }
            }
            if hours > h {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}

fn main() {}
