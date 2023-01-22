struct Solution {}
impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        // [1,2] 1.5
        // [1,2,3] 2
        // [1,2,3,4] 2.5
        // [1,2,3,4,5] 3
        // [1,2,3,4,5,6] 3.5
        let mut result = vec![];

        let mut seq_len = 2;
        let mut odd = false;

        loop {
            if target / seq_len < (seq_len + 1) / 2 {
                break;
            }

            if odd {
                if target % seq_len == 0 {
                    let mut v = Vec::with_capacity(seq_len as usize);
                    for i in 0..seq_len {
                        v.push(i + target / seq_len - seq_len / 2);
                    }
                    result.push(v);
                }
            } else {
                if target % seq_len == seq_len / 2 {
                    let mut v = Vec::with_capacity(seq_len as usize);
                    for i in 0..seq_len {
                        v.push(i + target / seq_len - seq_len / 2 + 1);
                    }
                    result.push(v);
                }
            }

            seq_len += 1;
            odd = !odd;
        }
        result.reverse();
        return result;
    }
}

fn main() {
    Solution::find_continuous_sequence(9);
}
