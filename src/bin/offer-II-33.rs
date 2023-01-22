struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::with_capacity(256);
        for s in strs {
            let mut count = [0; 26];
            s.as_bytes()
                .iter()
                .for_each(|&ch| count[(ch - b'a') as usize] += 1);
            map.entry(count).or_insert(vec![]).push(s);
        }
        map.into_iter().map(|(_, vec)| vec).collect()
    }
}

fn main() {
    // println!("{:?}", Solution);
}
