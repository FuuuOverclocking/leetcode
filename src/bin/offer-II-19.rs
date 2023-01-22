struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut l = 0;
        let mut r = s.len() - 1;

        loop {
            if s[l] != s[r] {
                return valid_palindrome(&s[(l + 1)..(r + 1)]) || valid_palindrome(&s[l..r]);
            }
            l += 1;
            r -= 1;
            if l >= r {
                return true;
            }
        }

        fn valid_palindrome(s: &[u8]) -> bool {
            let mut l: i32 = 0;
            let mut r: i32 = (s.len() - 1) as i32;

            loop {
                if s[l as usize] != s[r as usize] {
                    return false;
                }
                l += 1;
                r -= 1;
                if l >= r {
                    return true;
                }
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::valid_palindrome("abca".to_string()));
}
