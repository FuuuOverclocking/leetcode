struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: &[u8]) -> i32 {
        struct Elem {
            idx: usize,
            val: u8,
            l: usize,
        }

        let mut ret = 0;
        let mut s: Vec<Elem> = Vec::with_capacity(heights.len());

        for (idx, &val) in heights.iter().enumerate() {
            while !s.is_empty() && s.last().unwrap().val >= val {
                let el = s.pop().unwrap();
                ret = ret.max((el.val as i32) * (idx - el.l) as i32);
            }
            let l = if s.is_empty() {
                0
            } else {
                s.last().unwrap().idx + 1
            };
            s.push(Elem { idx, val, l });
        }
        let idx = heights.len();
        while let Some(el) = s.pop() {
            ret = ret.max((el.val as i32) * (idx - el.l) as i32);
        }

        ret
    }

    pub fn maximal_rectangle(mut matrix: Vec<String>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        if n == 0 {
            return 0;
        }

        for j in 0..n {
            let mut count = 0;
            for i in 0..m {
                let ch = unsafe { &mut matrix[i].as_bytes_mut()[j] };
                if *ch == b'0' {
                    count = 0;
                } else {
                    count += 1;
                }
                *ch = count;
            }
        }

        let mut ret = 0;
        for i in 0..m {
            ret =
                ret.max(Solution::largest_rectangle_area(matrix[i].as_bytes()))
        }

        ret
    }
}

fn main() {
    println!("{:?}", Solution::maximal_rectangle(vec!["0".to_owned(),]));
}
