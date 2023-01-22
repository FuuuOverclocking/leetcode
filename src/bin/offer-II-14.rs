struct Solution {}

impl Solution {
    pub fn check_inclusion(mut s1: String, mut s2: String) -> bool {
        let (s1, s2) = unsafe {
            let s1 = s1.as_bytes_mut();
            let s2 = s2.as_bytes_mut();

            s1.iter_mut().for_each(|ch| *ch -= b'a');
            s2.iter_mut().for_each(|ch| *ch -= b'a');

            (s1 as &[u8], s2 as &[u8])
        };

        if s1.len() > s2.len() {
            return false;
        }

        let mut cnt = [0_i32; 26];
        for i in 0..s1.len() {
            cnt[s1[i] as usize] -= 1;
            cnt[s2[i] as usize] += 1;
        }

        let mut diff = 0;
        for num in cnt {
            if num != 0 {
                diff += 1;
            }
        }
        if diff == 0 {
            return true;
        }
        for i in s1.len()..s2.len() {
            let ch_leave = s2[i - s1.len()] as usize;
            let ch_enter = s2[i] as usize;

            if ch_leave == ch_enter {
                continue;
            }

            if cnt[ch_leave] == 0 {
                diff += 1;
            }
            cnt[ch_leave] -= 1;
            if cnt[ch_leave] == 0 {
                diff -= 1;
            }

            if cnt[ch_enter] == 0 {
                diff += 1;
            }
            cnt[ch_enter] += 1;
            if cnt[ch_enter] == 0 {
                diff -= 1;
            }

            if diff == 0 {
                return true;
            }
        }

        false
    }
}

fn main() {
    // println!("{:?}", Solution);
}
