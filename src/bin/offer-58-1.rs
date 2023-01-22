struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .filter(|s| s.trim() != "")
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

fn main() {}
