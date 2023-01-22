struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = {
            let s = s.as_bytes();
            let mut new_s = Vec::with_capacity(s.len() * 2 + 3);
            new_s.push(b'^');
            new_s.push(b'#');
            s.iter().for_each(|&ch| {
                new_s.push(ch);
                new_s.push(b'#')
            });
            new_s.push(b'$');
            new_s
        };

        let mut f = vec![0; s.len()];
        let mut im = 0;
        let mut rm = 0;
        let mut ret = 0;

        for i in 1..(s.len() - 1) {
            f[i] = if i <= rm {
                f[2 * im - i].min(rm - i + 1)
            } else {
                1
            };
            while s[i + f[i]] == s[i - f[i]] {
                f[i] += 1;
            }
            if i + f[i] - 1 > rm {
                rm = i + f[i] - 1;
                im = i;
            }
            ret += f[i] / 2;
        }

        ret as i32
    }
}

fn main() {
    // println!("{:?}", Solution);
}
