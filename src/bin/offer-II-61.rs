use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ret = vec![];

        #[derive(PartialEq, Eq)]
        struct Elem {
            i_1: usize,
            i_2: usize,
            val_1: i32,
            val_2: i32,
        }
        impl PartialOrd for Elem {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                (self.val_1 + self.val_2).partial_cmp(&(other.val_1 + other.val_2))
            }
        }
        impl Ord for Elem {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                (self.val_1 + self.val_2).cmp(&(other.val_1 + other.val_2))
            }
        }

        let mut heap = BinaryHeap::<Reverse<Elem>>::new();

        for i in 0..k {
            if i == nums1.len() {
                break;
            }
            heap.push(Reverse(Elem {
                i_1: i,
                i_2: 0,
                val_1: nums1[i],
                val_2: nums2[0],
            }));
        }

        let mut k = k;
        while let Some(Reverse(elem)) = heap.pop() {
            if k == 0 {
                break;
            }
            ret.push(vec![elem.val_1, elem.val_2]);
            if elem.i_2 + 1 < nums2.len() {
                heap.push(Reverse(Elem {
                    i_1: elem.i_1,
                    i_2: elem.i_2 + 1,
                    val_1: nums1[elem.i_1],
                    val_2: nums2[elem.i_2 + 1],
                }));
            }
            k -= 1;
        }

        ret
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
    );
}
