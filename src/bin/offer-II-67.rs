#[derive(Default)]
struct Trie {
    can_be_end: bool,
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, num: i32) {
        let mut node = self;
        for i in (0..31).rev() {
            let bit = (num >> i) & 1;
            node = node.children[bit as usize].get_or_insert(Default::default());
        }
        node.can_be_end = true;
    }

    fn find_max_xor(&self, num: i32) -> i32 {
        let mut node = self;
        let mut ret = 0;
        for i in (0..31).rev() {
            let bit = (num >> i) & 1;
            if node.children[(bit ^ 1) as usize].is_some() {
                ret |= 1 << i;
                node = node.children[(bit ^ 1) as usize].as_ref().unwrap();
            } else {
                node = node.children[bit as usize].as_ref().unwrap();
            }
        }
        ret
    }
}

struct Solution {}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let mut ret = 0;
        for num in nums {
            trie.insert(num);
            ret = ret.max(trie.find_max_xor(num));
        }
        ret
    }
}

fn main() {

}