#[derive(Default)]
struct Trie {
    can_be_end: bool,
    children: [Option<Box<Trie>>; 26],
}

struct Solution {}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::default();

        for word in words {
            let word = word.as_bytes();
            let mut node = &mut trie;
            for &ch in word.iter().rev() {
                let ch = (ch - b'a') as usize;
                node = node.children[ch].get_or_insert(Default::default());
            }
            node.can_be_end = true;
        }

        fn count(node: &Trie, curr_len: i32) -> i32 {
            let mut sum = 0;
            let mut is_leaf = true;
            for child in node.children.iter() {
                if let Some(child) = child.as_deref() {
                    is_leaf = false;
                    sum += count(child, curr_len + 1);
                }
            }

            if is_leaf {
                curr_len + 1
            } else {
                sum
            }
        }

        count(&trie, 0)
    }
}

fn main() {}
