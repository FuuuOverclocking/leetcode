struct Solution {}

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize;
        let mut ss = String::with_capacity(s.len() + n);
        ss.clone_from(&s);
        ss.push_str(&s[..n]);
        ss[n..].to_string()
    }
}

fn main() {}
