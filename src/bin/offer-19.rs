struct Solution {}

impl Solution {
    pub fn is_match(str: String, pattern: String) -> bool {
        let str = str.as_bytes();
        let pattern = pattern.as_bytes();

        fn is_match(str: &[u8], pattern: &[u8]) -> bool {
            if pattern.len() == 0 {
                return str.len() == 0;
            }
            let ch = pattern[0];
            let asterisk = if pattern.len() >= 2 {
                pattern[1] == b'*'
            } else {
                false
            };
            let pattern = if asterisk {
                &pattern[2..]
            } else {
                &pattern[1..]
            };
            if asterisk {
                if is_match(str, pattern) {
                    return true;
                }
                for i in 0..str.len() {
                    if ch == b'.' || ch == str[i] {
                        if is_match(&str[(i + 1)..], pattern) {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
                return false;
            } else {
                if str.len() == 0 {
                    return false;
                }
                if ch == b'.' || ch == str[0] {
                    return is_match(&str[1..], pattern);
                } else {
                    return false;
                }
            }
        }

        is_match(str, pattern)
    }
}

fn main() {}
