struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        #[derive(Clone, Copy)]
        struct Range(usize, usize);
        impl Range {
            pub fn len(&self) -> usize {
                self.1 - self.0
            }
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        if s.len() < t.len() {
            return "".to_string();
        }

        let mut min_range: Option<Range> = None;
        let mut range: Option<Range> = None;
        let mut dict = [false; 128];
        let mut cnt = [0_i32; 128];
        t.iter().for_each(|&ch| {
            dict[ch as usize] = true;
            cnt[ch as usize] += 1;
        });

        let mut diff = cnt
            .iter()
            .fold(0, |acc, &n| acc + if n != 0 { 1 } else { 0 });

        for r in 0..s.len() {
            let ch_r = s[r] as usize;

            if range.is_none() {
                if dict[ch_r] {
                    range = Some(Range(r, r + 1));
                    cnt[ch_r] -= 1;
                    if cnt[ch_r] == 0 {
                        diff -= 1;
                        if diff == 0 {
                            min_range = Some(Range(r, r + 1));
                        }
                    }
                }
                continue;
            }

            let range = range.as_mut().unwrap();
            range.1 = r + 1;
            if !dict[ch_r] {
                continue;
            }

            cnt[ch_r] -= 1;
            if cnt[ch_r] == 0 {
                diff -= 1;
                if diff == 0 {
                    min_range = min_range
                        .or_else(|| Some(range.clone()))
                        .and_then(|min_range| {
                            if range.len() < min_range.len() {
                                Some(range.clone())
                            } else {
                                Some(min_range)
                            }
                        });
                }
            }
            if diff == 0 {
                loop {
                    let ch_l = s[range.0] as usize;
                    if !dict[ch_l] {
                        range.0 += 1;
                        min_range = min_range
                            .or_else(|| Some(range.clone()))
                            .and_then(|min_range| {
                                if range.len() < min_range.len() {
                                    Some(range.clone())
                                } else {
                                    Some(min_range)
                                }
                            });

                        continue;
                    }
                    if cnt[ch_l] == 0 {
                        break;
                    }
                    cnt[ch_l] += 1;
                    range.0 += 1;
                    min_range = min_range
                        .or_else(|| Some(range.clone()))
                        .and_then(|min_range| {
                            if range.len() < min_range.len() {
                                Some(range.clone())
                            } else {
                                Some(min_range)
                            }
                        });
                }
            }
        }
        if let Some(min_range) = min_range {
            unsafe {
                String::from_utf8_unchecked(
                    s[min_range.0..min_range.1].to_vec(),
                )
            }
        } else {
            "".to_string()
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::min_window("a".to_string(), "b".to_string())
    );
}
