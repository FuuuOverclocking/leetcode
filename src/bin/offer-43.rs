struct Solution {}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        // e.g. n = 5102

        let mut result = 0;

        //         i=0
        // 5  1  0  2
        let mut i = 0;

        let mut tmp = n;

        loop {
            let digit = tmp % 10;
            let prefix = tmp / 10;

            if digit > 1 {
                // 1st loop

                // tmp    = 5  1  0  2
                // digit  = 5  1  0  [2]
                // prefix = 5  1  0
                //         i=0
                // 5  1  0  2

                // += 511 * 10^0
                result += (prefix + 1) * 10_i32.pow(i);
            } else if digit < 1 {
                // 2nd loop

                // tmp    = 5  1  0
                // digit  = 5  1  [0]
                // prefix = 5  1
                //      i=1
                // 5  1  0  2

                // += 51 * 10^1
                result += prefix * 10_i32.pow(i);
            } else {
                // 3rd loop

                // tmp    = 5  1
                // digit  = 5  [1]
                // prefix = 5
                //   i=2
                // 5  1  0  2

                // += 5 * 10^2
                result += prefix * 10_i32.pow(i);
                // += 5102 - 5100 + 1
                result += n - tmp * 10_i32.pow(i) + 1;
            }

            i += 1;
            tmp /= 10;
            if tmp == 0 {
                break;
            }
        }

        result
    }
}

fn main() {}
