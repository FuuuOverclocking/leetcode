struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut fast = &head as *const Option<Box<ListNode>>;
        let mut slow = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            for _ in 0..k {
                fast = &(*fast).as_ref()?.next;
            }
            while (*fast).is_some() {
                fast = &(*fast).as_ref()?.next;
                slow = &mut (*slow).as_mut()?.next;
            }
            (*slow).take()
        }
    }
}

fn main() {}
