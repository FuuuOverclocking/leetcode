#[derive(Default)]
struct Trie {
    can_be_end: bool,
    children: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let word = word.as_bytes();

        let mut node = self;
        for &ch in word {
            let ch = (ch - b'a') as usize;
            node = node.children[ch].get_or_insert(Default::default());
        }
        node.can_be_end = true;
    }
}

struct Solution {}
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();

        for root in dictionary {
            trie.insert(root);
        }
        let trie = trie;

        let sentence = sentence.as_bytes();
        let mut ret = Vec::with_capacity(sentence.len());

        let mut work = |word: &[u8], should_add_space: bool| {
            if should_add_space {
                ret.push(b' ');
            }

            let mut node = &trie;
            for (i, &ch) in word.iter().enumerate() {
                let ch_as_idx = (ch - b'a') as usize;
                if let Some(n) = node.children[ch_as_idx].as_deref() {
                    node = n;
                    if node.can_be_end {
                        ret.extend_from_slice(&word[..=i]);
                        return;
                    }
                } else {
                    break;
                }
            }
            ret.extend_from_slice(word);
        };
        let mut should_add_space = false;
        for word in sentence.split(|&ch| ch == b' ') {
            work(word, should_add_space);
            should_add_space = true;
        }

        unsafe { String::from_utf8_unchecked(ret) }
    }
}

fn main() {}
