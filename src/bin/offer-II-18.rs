struct Solution {}

impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        let s = unsafe { s.as_bytes_mut() };
        let mut l: i32 = 0;
        let mut r: i32 = (s.len() - 1) as i32;

        loop {
            // 当前 l,r 可能指向字母数字, 也可能指向其他
            loop {
                let ch = &mut s[l as usize];
                if b'A' <= *ch && *ch <= b'Z' {
                    *ch = *ch - b'A' + b'a';
                    break;
                }
                if b'a' <= *ch && *ch <= b'z' {
                    break;
                }
                if b'0' <= *ch && *ch <= b'9' {
                    break;
                }
                l += 1;
                if l > r {
                    break;
                }
            }
            loop {
                let ch = &mut s[r as usize];
                if b'A' <= *ch && *ch <= b'Z' {
                    *ch = *ch - b'A' + b'a';
                    break;
                }
                if b'a' <= *ch && *ch <= b'z' {
                    break;
                }
                if b'0' <= *ch && *ch <= b'9' {
                    break;
                }
                r -= 1;
                if l > r {
                    break;
                }
            }
            if l > r {
                return true;
            }
            if s[l as usize] != s[r as usize] {
                return false;
            }
            l += 1;
            r -= 1;
            if l > r {
                return true;
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::is_palindrome(" ".to_string()));
}
