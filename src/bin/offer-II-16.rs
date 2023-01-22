struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();

        let mut last_pos = vec![-1_i32; 128];
        let mut start_at = 0_usize;
        let mut max = 0;

        for i in 0..s.len() {
            let ch = s[i] as usize;
            let pos = last_pos[ch];

            if pos < start_at as i32 {
                last_pos[ch] = i as i32;
                continue;
            }
            max = max.max(i - start_at);
            start_at = (pos + 1) as usize;
            last_pos[ch] = i as i32;
        }

        max = max.max(s.len() - start_at);

        max as i32
    }
}

fn main() {
    // println!("{:?}", Solution);
}
