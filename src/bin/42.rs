struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut water = 0; // 当前水量, 随着迭代持续修正
        let mut h = 0; // 当前已知 l,r 间最高水高

        // 初始化
        h = height[l].min(height[r]);
        water = ((r - l - 1) as i32) * h;

        // l,r 相互靠近, 直到重合
        loop {
            // 移动双指针之一
            let move_l = if height[l] <= height[r] {
                l += 1;
                true
            } else {
                r -= 1;
                false
            };
            if l >= r {
                break;
            } else {
                // 减去多算的黑色部分
                let extra = if move_l { height[l] } else { height[r] };
                water -= extra.min(h);
            }
            // 更新 h, water
            let new_h = h.max(height[l].min(height[r]));
            water += (new_h - h) * (r - l - 1) as i32;
            h = new_h;
        }

        water
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    );
}
