struct Solution {}

impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut root = i32::MAX;
        for i in (0..postorder.len()).rev() {
            if postorder[i] > root {
                return false;
            }

            while !stack.is_empty() && *stack.last().unwrap() > postorder[i] {
                root = stack.pop().unwrap();
            }

            stack.push(postorder[i]);
        }
        return true;
    }
}

fn main() {}
