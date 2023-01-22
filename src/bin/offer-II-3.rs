struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ret = Vec::with_capacity(n + 1);
        
        ret.push(0);
        if n == 0 {
            return ret;
        }

        ret.push(1);
        if n == 1 {
            return ret;
        }

        let mut curr = 2;
        let mut i = 2;
        loop {
            ret.push(ret[i - curr] + 1);

            i += 1;
            if i == n + 1 {
                break;
            }
            if i == curr * 2 {
                curr *= 2;
            }
        }

        ret
    }
}

fn main() {
    println!("{:?}", Solution::count_bits(10));
}
