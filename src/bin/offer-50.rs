struct Solution {}

#[derive(Copy, Clone, PartialEq)]
enum ChState {
    NotPresent,
    FirstAt(usize),
    Repeated,
}

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let s = s.as_bytes();
        let mut map = [ChState::NotPresent; 127];

        for (i, &ch) in s.iter().enumerate() {
            match map[ch as usize] {
                ChState::NotPresent => {
                    map[ch as usize] = ChState::FirstAt(i);
                }
                ChState::FirstAt(_) => {
                    map[ch as usize] = ChState::Repeated;
                }
                _ => {}
            }
        }

        let mut first = (b' ', usize::MAX);
        for (ch, state) in map.iter().enumerate() {
            if let ChState::FirstAt(pos) = state {
                if first.1 > *pos {
                    first = (ch as u8, *pos);
                }
            }
        }

        return first.0 as char;
    }
}

fn main() {
    Solution::first_uniq_char("abcde".into());
}
