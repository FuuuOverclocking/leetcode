use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut ret = Self {
            k: k as usize,
            heap: BinaryHeap::new(),
        };
        for ele in nums {
            ret.add(ele);
        }
        ret
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop().unwrap();
        }
        (*self.heap.peek().unwrap()).0
    }
}

fn main() {
    // println!("{:?}", );
}
