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

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();

        let mut node = self;

        for &ch in word {
            let ch = (ch - b'a') as usize;
            if node.children[ch].is_none() {
                return false;
            }
            node = node.children[ch].as_deref().unwrap();
        }

        node.can_be_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let word = prefix.as_bytes();

        let mut node = self;

        for &ch in word {
            let ch = (ch - b'a') as usize;
            if node.children[ch].is_none() {
                return false;
            }
            node = node.children[ch].as_deref().unwrap();
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {}
