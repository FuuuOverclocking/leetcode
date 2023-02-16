#[derive(Default)]
struct Trie {
    can_be_end: bool,
    val: i32,
    count: i32,
    children: [Option<Box<Trie>>; 26],
}

#[derive(Default)]
struct MapSum {
    root: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, key: String, val: i32) {
        let word = key.as_bytes();
        let mut node = &mut self.root;
        let mut arr = vec![node as *mut Trie];
        for &ch in word.iter() {
            let ch = (ch - b'a') as usize;
            node = node.children[ch].get_or_insert(Default::default());
            arr.push(node as *mut Trie);
        }

        if !node.can_be_end {
            node.can_be_end = true;
            node.val = val;
            for &node in arr.iter() {
                unsafe {
                    (*node).count += val;
                }
            }
        } else {
            let diff = val - node.val;
            node.val = val;
            for &node in arr.iter() {
                unsafe {
                    (*node).count += diff;
                }
            }
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let prefix = prefix.as_bytes();

        let mut node = &self.root;
        for &ch in prefix {
            let ch = (ch - b'a') as usize;
            node = match node.children[ch].as_deref() {
                Some(node) => node,
                None => return 0,
            };
        }

        node.count
    }
}

fn main() {
    let mut map = MapSum::new();
    map.insert("apple".to_string(), 3);
    map.insert("app".to_string(), 2);
    map.insert("apple".to_string(), 5);
    map.insert("apple".to_string(), 1);
}
