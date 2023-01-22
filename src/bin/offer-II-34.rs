struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let map = {
            let mut map = [0; 26];
            for (i, &ch) in order.as_bytes().iter().enumerate() {
                let ch = (ch - b'a') as usize;
                map[ch] = i;
            }
            map
        };
        if words.len() == 1 {
            return true;
        }
        for i in 0..(words.len() - 1) {
            let curr = words[i].as_bytes();
            let next = words[i + 1].as_bytes();
            if !le(curr, next, &map) {
                return false;
            }
        }

        fn le(a: &[u8], b: &[u8], map: &[usize; 26]) -> bool {
            let len = a.len().max(b.len());
            for i in 0..len {
                if i == a.len() {
                    return true;
                }
                if i == b.len() {
                    return false;
                }
                let a = map[(a[i] - b'a') as usize];
                let b = map[(b[i] - b'a') as usize];
                if a < b {
                    return true;
                }
                if a > b {
                    return false;
                }
            }
            true
        }

        true
    }
}

fn main() {
    // println!("{:?}", Solution);
}
