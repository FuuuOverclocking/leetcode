struct Solution {}

impl Solution {
    pub fn str_to_int(str: String) -> i32 {
        let str = str.as_bytes();

        let mut state = 0;
        let mut isPositive = true;
        let mut digits = vec![];

        for &ch in str {
            match state {
                0 => match ch {
                    b' ' => {}
                    b'+' => {
                        state = 1;
                    }
                    b'-' => {
                        state = 1;
                        isPositive = false;
                    }
                    b'0'..=b'9' => {
                        digits.push(ch - b'0');
                        state = 2;
                    }
                    _ => {
                        return 0;
                    }
                },
                1 => match ch {
                    b'0'..=b'9' => {
                        digits.push(ch - b'0');
                        state = 2;
                    }
                    _ => {
                        return 0;
                    }
                },
                2 => match ch {
                    b'0'..=b'9' => {
                        digits.push(ch - b'0');
                    }
                    _ => {
                        state = 3;
                    }
                },
                _ => {}
            }
        }
        if state != 2 && state != 3 {
            return 0;
        }

        let mut ans = if isPositive {
            digits[0] as i32
        } else {
            -(digits[0] as i32)
        };

        for &d in digits.iter().skip(1) {
            ans = ans.saturating_mul(10);
            if isPositive {
                ans = ans.saturating_add(d as i32);
            } else {
                ans = ans.saturating_sub(d as i32);
            }
        }

        ans
    }
}

fn main() {}
