struct Solution {}
impl Solution {
    pub fn divide(mut a: i32, mut b: i32) -> i32 {
        // a / b = c
        if b == 0 {
            return i32::MAX;
        }
        if a == 0 {
            return 0;
        }
        if a == i32::MIN {
            if b == -1 {
                return i32::MAX;
            }
            if b == 1 {
                return a;
            }
        }
        if b == i32::MIN {
            if a == i32::MIN {
                return 1;
            }
            return 0;
        };

        let mut rev = false;
        if a > 0 {
            rev = !rev;
            a = -a;
        }
        if b > 0 {
            rev = !rev;
            b = -b;
        }

        let mut arr = Vec::new();
        arr.push(b);
        loop {
            if *arr.last().unwrap() < a - arr.last().unwrap() {
                break;
            }
            arr.push(arr.last().unwrap() + arr.last().unwrap());
        }

        let mut ans = 0;
        for i in (0..arr.len()).rev() {
            let num = arr[i];
            if num >= a {
                ans += 1 << i;
                a -= num;
            }
        }

        if rev {
            -ans
        } else {
            ans
        }
    }
}

fn main() {
    Solution::divide(-2147483648, 2);
}
