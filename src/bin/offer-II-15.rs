struct Solution {}

impl Solution {
    pub fn find_anagrams(mut s2: String, mut s1: String) -> Vec<i32> {
        let (s1, s2) = unsafe {
            let s1 = s1.as_bytes_mut();
            let s2 = s2.as_bytes_mut();

            s1.iter_mut().for_each(|ch| *ch -= b'a');
            s2.iter_mut().for_each(|ch| *ch -= b'a');

            (s1 as &[u8], s2 as &[u8])
        };
        let mut ret = vec![];

        if s1.len() > s2.len() {
            return ret;
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
            ret.push(0);
        }
        for i in s1.len()..s2.len() {
            let ch_leave = s2[i - s1.len()] as usize;
            let ch_enter = s2[i] as usize;

            if ch_leave == ch_enter {
                if diff == 0 {
                    ret.push((i - s1.len() + 1) as i32);
                }
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
                ret.push((i - s1.len() + 1) as i32);
            }
        }

        ret
    }
}

fn main() {
    // println!("{:?}", Solution);
}
