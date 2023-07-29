struct Solution {}
impl Solution {
    pub fn max_product(mut words: Vec<String>) -> i32 {
        let len_arr =
            words.iter().map(|w| w.as_bytes().len()).collect::<Vec<_>>();
        let words = words
            .iter()
            .map(|word| {
                let word = word.as_bytes();
                let mut k: u32 = 0;
                for &ch in word {
                    let ch = ch - b'a';
                    k |= 1 << ch;
                }
                k
            })
            .collect::<Vec<_>>();

        let mut max = 0;
        for i in 0..(words.len() - 1) {
            for j in (i + 1)..words.len() {
                if words[i] & words[j] == 0 {
                    max = max.max((len_arr[i] * len_arr[j]) as i32);
                }
            }
        }

        max
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::max_product(vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ])
    );
}
