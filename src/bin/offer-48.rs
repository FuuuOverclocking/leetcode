struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp;
        use std::collections::hash_map::HashMap;

        let s = s.as_bytes();
        if s.len() == 0 {
            return 0;
        }

        let mut map = HashMap::<u8, usize>::with_capacity(128);
        map.insert(s[0], 0);
        let mut window = (0_usize, 1_usize);
        let mut max_len = 1;

        for i in 1..s.len() {
            let ch = s[i];

            if let Some(&pos) = map.get(&ch) {
                if pos >= window.0 {
                    window.0 = pos + 1;
                }
            }

            window.1 = i + 1;
            map.insert(ch, i);
            max_len = cmp::max(max_len, window.1 - window.0);
        }

        return max_len as i32;
    }
}

fn main() {
    Solution::length_of_longest_substring("abba".to_string());
}
