struct Solution {}

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();

        let mut ret = 0;
        let mut expect_zero = true;
        let mut zero_i_see = 0;
        let mut one_i_see = 0;

        for &ch in s {
            if ch == b'0' {
                if expect_zero {
                    zero_i_see += 1;
                } else {
                    expect_zero = true;
                    ret = ret.max(zero_i_see.min(one_i_see) * 2);
                    zero_i_see = 1;
                    one_i_see = 0;
                }
            } else {
                if expect_zero {
                    expect_zero = false;
                }
                one_i_see += 1;
            }
        }
        if !expect_zero {
            ret = ret.max(zero_i_see.min(one_i_see) * 2);
        }

        ret
    }
}

fn main() {}
